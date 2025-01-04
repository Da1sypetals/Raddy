use approx::assert_abs_diff_eq;
use faer::{prelude::SpSolver, sparse::SparseColMat, Col};
use nalgebra::SVector;
use raddy::{make::val, sparse::objective::Objective, types::advec};

struct SpringEnergy {
    k: f64,
    restlen: f64,
}

// 2d * 2nodes = 4dof
impl Objective<4> for SpringEnergy {
    type EvalArgs = ();
    fn eval(&self, variables: &raddy::types::advec<4, 4>, _: &()) -> raddy::Ad<4> {
        let p1 = advec::<4, 2>::new(variables[0].clone(), variables[1].clone());
        let p2 = advec::<4, 2>::new(variables[2].clone(), variables[3].clone());

        let len = (p2 - p1).norm();
        // Hooke's law
        let potential = val::scalar(0.5 * self.k) * (len - val::scalar(self.restlen)).powi(2);

        potential
    }
}

fn main() {
    // todo!("This example is undone");

    let springs = vec![[0, 1, 2, 3], [2, 3, 4, 5], [0, 1, 4, 5]];
    let x0 = faer::col::from_slice(&[0.0, 0.0, 0.001, 0.0, 0.001, 0.01]).to_owned();

    let obj = SpringEnergy {
        k: 10000.0,
        restlen: 1.0,
    };

    let mut i = 0;
    let mut x = x0.clone();
    let mut dir: Col<f64>;
    // Newton Raphson
    while {
        let grad = obj.grad(&x, &springs, &());
        let mut hesstrip = obj.hess_trips(&x, &springs, &());
        for i in 0..6 {
            hesstrip.push((i, i, 1.0));
        }
        let hess = SparseColMat::try_new_from_triplets(6, 6, &hesstrip).unwrap();

        // wtf matrix is not invertible?
        dir = hess.sp_lu().unwrap().solve(-&grad);

        dir.norm_l2() > 1e-8
    } {
        x += dir;

        i += 1;

        let p1 = SVector::<f64, 2>::new(x[0], x[1]);
        let p2 = SVector::<f64, 2>::new(x[2], x[3]);
        let p3 = SVector::<f64, 2>::new(x[4], x[5]);

        println!("\nIter {}", i);
        println!("Len 1 = {}", (p2 - p1).norm());
        println!("Len 2 = {}", (p3 - p2).norm());
        println!("Len 3 = {}", (p3 - p1).norm());
    }

    println!("\nFinal potential: {}", obj.value(&x, &springs, &()));
    let p1 = SVector::<f64, 2>::new(x[0], x[1]);
    let p2 = SVector::<f64, 2>::new(x[2], x[3]);
    let p3 = SVector::<f64, 2>::new(x[4], x[5]);

    assert_abs_diff_eq!((p2 - p1).norm(), 1.0, epsilon = 1e-4);
    assert_abs_diff_eq!((p3 - p2).norm(), 1.0, epsilon = 1e-4);
    assert_abs_diff_eq!((p3 - p1).norm(), 1.0, epsilon = 1e-4);
}
