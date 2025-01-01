#![allow(unused)]

use crate::make;
use crate::misc::symbolic_3::{grad_mmsin2, hess__mmsin2};
use crate::{
    misc::symbolic_1::{grad_det3, grad_det4, hess_det3, hess_det4},
    test::EPS,
    Ad, GetValue,
};
use approx::assert_abs_diff_eq;
use na::{Const, SMatrix, SVector};
use rand::{thread_rng, Rng};

#[test]
fn test_norm_1() {
    const N_TEST_MAT_1: usize = 30;

    let mut rng = thread_rng();
    // let vals = &[1.2, -4.2, 2.4, 0.4];
    let vals: &[f64] = &(0..N_TEST_MAT_1)
        .map(|_| rng.gen_range(-4.0..4.0))
        .collect::<Vec<_>>();

    // core logic #########################################################
    let s: SVector<Ad<N_TEST_MAT_1>, N_TEST_MAT_1> = make::vec(vals);
    let z = s.norm();
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
    let s: SVector<Ad<N_VEC_2>, N_VEC_2> = make::vec(vals);
    let z = s.clone().reshape_generic(NaConst {}, NaConst {});
    let tr = (z.transpose() * z).trace();
    // core logic ends ####################################################

    // dbg!(&tr.grad());
    let expected_grad = s.scale(Ad::inactive_scalar(2.0));
    let g_diff = (expected_grad.value() - tr.grad()).norm_squared();
    assert_abs_diff_eq!(g_diff, 0.0, epsilon = EPS);

    assert_eq!(tr.hess, SMatrix::<f64, 9, 9>::identity() * 2.0);
}

#[test]
fn test_det3() {
    const N_TEST_MAT_3: usize = 3;
    type NaConst = Const<N_TEST_MAT_3>;
    const N_VEC_3: usize = N_TEST_MAT_3 * N_TEST_MAT_3;

    let mut rng = thread_rng();
    // let vals = &[1.2, -4.2, 2.4, 0.4];
    let vals: &[f64] = &(0..N_VEC_3)
        .map(|_| rng.gen_range(-4.0..4.0))
        .collect::<Vec<_>>();

    // core logic #########################################################
    let s: SVector<Ad<N_VEC_3>, N_VEC_3> = make::vec(vals);
    let z = s
        .clone()
        // This reshape is COL MAJOR!!!!!!!!!!!!!
        .reshape_generic(NaConst {}, NaConst {})
        .transpose();

    let det = z.determinant();
    // core logic ends ####################################################

    // dbg!(&tr.grad());
    let expected_grad = grad_det3(
        vals[0], vals[1], vals[2], vals[3], vals[4], vals[5], vals[6], vals[7], vals[8],
    );
    let g_diff = (expected_grad - det.grad()).norm_squared();
    assert_abs_diff_eq!(g_diff, 0.0, epsilon = EPS);

    let expected_hess = hess_det3(
        vals[0], vals[1], vals[2], vals[3], vals[4], vals[5], vals[6], vals[7], vals[8],
    );
    assert_eq!(det.hess, expected_hess);
}

#[test]
fn test_det4() {
    const N_TEST_MAT_4: usize = 4;
    type NaConst = Const<N_TEST_MAT_4>;
    const N_VEC_4: usize = N_TEST_MAT_4 * N_TEST_MAT_4;

    let mut rng = thread_rng();
    // let vals = &[1.2, -4.2, 2.4, 0.4];
    let vals: &[f64] = &(0..N_VEC_4)
        .map(|_| rng.gen_range(-4.0..4.0))
        .collect::<Vec<_>>();

    // core logic #########################################################
    let s: SVector<Ad<N_VEC_4>, N_VEC_4> = make::vec(vals);
    let z = s
        .clone()
        // This reshape is COL MAJOR!!!!!!!!!!!!!
        .reshape_generic(NaConst {}, NaConst {})
        .transpose();

    let det = z.determinant();
    // core logic ends ####################################################

    // dbg!(&tr.grad());
    let expected_grad = grad_det4(
        vals[0], vals[1], vals[2], vals[3], vals[4], vals[5], vals[6], vals[7], vals[8], vals[9],
        vals[10], vals[11], vals[12], vals[13], vals[14], vals[15],
    );
    let g_diff = (expected_grad - det.grad()).norm_squared();
    assert_abs_diff_eq!(g_diff, 0.0, epsilon = EPS);

    let expected_hess = hess_det4(
        vals[0], vals[1], vals[2], vals[3], vals[4], vals[5], vals[6], vals[7], vals[8], vals[9],
        vals[10], vals[11], vals[12], vals[13], vals[14], vals[15],
    );
    let h_diff = (det.hess - expected_hess).norm_squared();
    assert_abs_diff_eq!(h_diff, 0.0, epsilon = EPS);
}

#[test]
fn test_mm() {
    const N_TEST_MAT_5: usize = 2;
    type NaConst = Const<N_TEST_MAT_5>;
    const N_VEC_5: usize = N_TEST_MAT_5 * N_TEST_MAT_5;

    let mut rng = thread_rng();

    for i in 0..100 {
        // let vals = &[1.2, -4.2, 2.4, 0.4];
        let vals: &[f64] = &(0..N_VEC_5)
            .map(|_| rng.gen_range(-4.0..4.0))
            .collect::<Vec<_>>();

        // core logic #########################################################
        let s: SVector<Ad<N_VEC_5>, N_VEC_5> = make::vec(vals);
        let mut z = s
            .clone()
            // This reshape is COL MAJOR!!!!!!!!!!!!!
            .reshape_generic(NaConst {}, NaConst {})
            .transpose();

        let w = z.clone();
        z.apply(|x| *x = x.sin());

        let res = (z * w).norm().cos();
        // core logic ends ####################################################

        let dg = (res.grad() - grad_mmsin2(vals[0], vals[1], vals[2], vals[3])).norm_squared();
        assert_abs_diff_eq!(dg, 0.0, epsilon = EPS);

        let dh = (res.hess() - hess__mmsin2(vals[0], vals[1], vals[2], vals[3])).norm_squared();
        assert_abs_diff_eq!(dh, 0.0, epsilon = EPS);

        println!("Test mm iter {}", i);
    }
}
