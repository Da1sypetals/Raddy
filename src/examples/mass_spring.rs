use faer::{prelude::SpSolver, sparse::SparseColMat, Col};
use raddy::{
    sparse::{function::ObjectiveFunction, objective::Objective},
    types::{advec, vec},
    Ad,
};

const EPS: f64 = 1e-10;

struct SpringEnergy {
    k: f64,
    restlen: f64,
}

// 2d * 2nodes = 4dof
impl ObjectiveFunction<4> for SpringEnergy {
    fn eval(&self, variables: &raddy::types::advec<4, 4>) -> raddy::Ad<4> {
        let p1 = advec::<4, 2>::new(variables[0].clone(), variables[1].clone());
        let p2 = advec::<4, 2>::new(variables[2].clone(), variables[3].clone());

        let len = (p2 - p1).norm();
        // Hooke's law
        let potential =
            Ad::inactive_scalar(0.5 * self.k) * (len - Ad::inactive_scalar(self.restlen)).powi(2);

        potential
    }
}

fn main() {
    let springs = vec![[0, 1, 2, 3], [2, 3, 4, 5], [0, 1, 4, 5]];
    let x0 = faer::col::from_slice(&[0.0, 0.0, 2.0, 0.0, 1.0, 2.0]).to_owned();

    let obj = Objective::new(
        SpringEnergy {
            k: 10000.0,
            restlen: 1.0,
        },
        springs,
    );

    let mut i = 0;
    let mut x = x0.clone();
    let mut dir: Col<f64>;
    while {
        let grad = obj.grad(&x);
        let mut hesstrip = obj.hess_trips(&x);
        // for i in 0..6 {
        //     hesstrip.push((i, i, 1.0));
        // }
        let hess = SparseColMat::try_new_from_triplets(6, 6, &hesstrip).unwrap();

        dbg!(hess.to_dense().determinant());

        // wtf matrix is not invertible?
        dir = hess.sp_lu().unwrap().solve(-&grad);

        dir.norm_l2() > 1e-8
    } {
        x += dir;

        i += 1;

        let p1 = vec::<2>::new(x[0], x[1]);
        let p2 = vec::<2>::new(x[2], x[3]);
        let p3 = vec::<2>::new(x[4], x[5]);

        println!("\nIter {}", i);
        println!("Len 1 = {}", (p2 - p1).norm());
        println!("Len 2 = {}", (p3 - p2).norm());
        println!("Len 3 = {}", (p3 - p1).norm());
    }

    println!("Current potential: {}", obj.value(&x));
}
