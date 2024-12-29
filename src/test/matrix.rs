#![allow(unused)]

use approx::assert_abs_diff_eq;
use na::{Const, SMatrix, SVector};
use rand::{thread_rng, Rng};

use crate::{norms::AdMatrixOps, test::EPS, Ad, GetValue};

#[test]
fn test_norm_1() {
    const N_TEST_MAT_1: usize = 30;

    let mut rng = thread_rng();
    // let vals = &[1.2, -4.2, 2.4, 0.4];
    let vals: &[f64] = &(0..N_TEST_MAT_1)
        .map(|_| rng.gen_range(-4.0..4.0))
        .collect::<Vec<_>>();

    // core logic #########################################################
    let s: SVector<Ad<N_TEST_MAT_1>, N_TEST_MAT_1> = Ad::active_from_slice(vals);
    let z = s.l2_norm();
    // core logic ends ####################################################

    // Here's how we might compute the expected gradient for L2 norm:
    // The gradient of the L2 norm with respect to x_i is x_i / ||x||
    let norm = (vals.iter().map(|&x| x * x).sum::<f64>()).sqrt();
    let expected_grad = SVector::<f64, N_TEST_MAT_1>::from_row_slice(
        &vals.iter().map(|&x| x / norm).collect::<Vec<f64>>(),
    );

    // Check if the gradients are close enough to the expected values
    let g_diff = (expected_grad - z.grad()).norm_squared();
    assert_abs_diff_eq!(g_diff, 0.0, epsilon = EPS);

    let h = z.hess;
    let expected_hess = (SMatrix::<f64, N_TEST_MAT_1, N_TEST_MAT_1>::identity()
        - expected_grad * expected_grad.transpose())
        / norm;
    let h_diff = (h - expected_hess).norm_squared();
    assert_abs_diff_eq!(h_diff, 0.0, epsilon = EPS);

    println!("Grad difference: {g_diff}\nHessian Difference: {h_diff}");
}

#[test]
fn test_norm_2() {
    const N_TEST_MAT_2: usize = 3;
    type NaConst = Const<N_TEST_MAT_2>;
    const N_VEC_2: usize = N_TEST_MAT_2 * N_TEST_MAT_2;

    let mut rng = thread_rng();
    // let vals = &[1.2, -4.2, 2.4, 0.4];
    let vals: &[f64] = &(0..N_VEC_2)
        .map(|_| rng.gen_range(-4.0..4.0))
        .collect::<Vec<_>>();

    // core logic #########################################################
    let s: SVector<Ad<N_VEC_2>, N_VEC_2> = Ad::active_from_slice(vals);
    let z = s.clone().reshape_generic(NaConst {}, NaConst {});
    let tr = (z.transpose() * z).trace();
    // core logic ends ####################################################

    // dbg!(&tr.grad());
    let expected_grad = s.scale(2.0);
    let g_diff = (expected_grad.value() - tr.grad()).norm_squared();
    assert_abs_diff_eq!(g_diff, 0.0, epsilon = EPS);

    assert_eq!(tr.hess, SMatrix::<f64, 9, 9>::identity() * 2.0);
}
