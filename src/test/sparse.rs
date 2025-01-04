use crate::{
    make::val,
    misc::symbolic_massspring::{
        spring3_energy, spring3_grad, spring3_hess, spring4_energy, spring4_grad, spring4_hess,
    },
    sparse::objective::Objective,
    test::EPS,
    types::advec,
    Ad,
};
use approx::assert_abs_diff_eq;
use faer::sparse::SparseColMat;
use na::SVector;

struct SpringEnergy {
    k: f64,
    restlen: f64,
}

// 2d * 2nodes = 4dof
impl Objective<4> for SpringEnergy {
    type EvalArgs = ();
    fn eval(&self, variables: &advec<4, 4>, _: &()) -> Ad<4> {
        let p1 = advec::<4, 2>::new(variables[0].clone(), variables[1].clone());
        let p2 = advec::<4, 2>::new(variables[2].clone(), variables[3].clone());

        let len = (p2 - p1).norm();
        // Hooke's law
        let potential = val::scalar(0.5 * self.k) * (len - val::scalar(self.restlen)).powi(2);

        potential
    }
}

#[test]
fn test_mass_spring_3() {
    // todo!("This example is undone");

    let springs = vec![[0, 1, 2, 3], [2, 3, 4, 5], [0, 1, 4, 5]];
    let x0 = faer::col::from_slice(&[0.0, 0.0, 1.6, 0.0, 0.8, 0.6]).to_owned();

    let k = 1.0;
    let restlen = 1.0;

    let obj = SpringEnergy { k, restlen };

    let computed = obj.compute(&x0, &springs, &());

    let e = computed.value;
    let ee = spring3_energy(k, restlen, x0[0], x0[1], x0[2], x0[3], x0[4], x0[5]);
    assert_abs_diff_eq!(e, ee, epsilon = EPS);

    let eg = spring3_grad(k, restlen, x0[0], x0[1], x0[2], x0[3], x0[4], x0[5]);
    let g = SVector::<f64, 6>::from_row_slice(computed.grad.as_slice());
    let gd = (eg - g).norm_squared();
    assert_abs_diff_eq!(gd, 0.0, epsilon = EPS);

    let eh_static = spring3_hess(k, restlen, x0[0], x0[1], x0[2], x0[3], x0[4], x0[5]);
    let eh_dyn = faer::mat::from_column_major_slice(eh_static.as_slice(), 6, 6);
    let h = SparseColMat::try_new_from_triplets(6, 6, &computed.hess_trips)
        .unwrap()
        .to_dense();
    let hd = (eh_dyn - h).squared_norm_l2();

    assert_abs_diff_eq!(hd, 0.0, epsilon = EPS);
}

#[test]
fn test_mass_spring_4() {
    // todo!("This example is undone");

    let springs = vec![[0, 1, 2, 3], [2, 3, 4, 5], [4, 5, 6, 7], [6, 7, 0, 1]];
    let x0 = faer::col::from_slice(&[0.0, 0.0, 4.0, 1.0, 3.0, 2.0, 1.0, 5.0]).to_owned();

    const NDOF: usize = 8;

    let k = 1.0;
    let restlen = 1.0;

    let obj = SpringEnergy { k, restlen };

    let computed = obj.compute(&x0, &springs, &());

    let e = computed.value;
    let ee = spring4_energy(
        k, restlen, x0[0], x0[1], x0[2], x0[3], x0[4], x0[5], x0[6], x0[7],
    );
    assert_abs_diff_eq!(e, ee, epsilon = EPS);

    let eg = spring4_grad(
        k, restlen, x0[0], x0[1], x0[2], x0[3], x0[4], x0[5], x0[6], x0[7],
    );
    let g = SVector::<f64, NDOF>::from_row_slice(computed.grad.as_slice());
    let gd = (eg - g).norm_squared();
    assert_abs_diff_eq!(gd, 0.0, epsilon = EPS);

    let eh_static = spring4_hess(
        k, restlen, x0[0], x0[1], x0[2], x0[3], x0[4], x0[5], x0[6], x0[7],
    );
    let eh_dyn = faer::mat::from_column_major_slice(eh_static.as_slice(), NDOF, NDOF);
    let h = SparseColMat::try_new_from_triplets(NDOF, NDOF, &computed.hess_trips)
        .unwrap()
        .to_dense();
    let hd = (eh_dyn - h).squared_norm_l2();

    assert_abs_diff_eq!(hd, 0.0, epsilon = EPS);
}
