#![allow(unused)]

use crate::{
    test::{
        symbolic::{
            grad_0, grad_1, grad_2, grad_3, grad_alpha, grad_beta, grad_costan, grad_kappa, hess_0,
            hess_1, hess_2, hess_alpha, hess_beta, hess_costan, hess_kappa,
        },
        BIG_EPS, EPS, RELRATIO,
    },
    Ad, GetValue,
};
use approx::assert_abs_diff_eq;
use nalgebra::{Const, DimName, Matrix3, SMatrix, SVector, U3, U5};
use rand::{thread_rng, Rng};
use std::{
    any::Any,
    ops::{Add, Div, Mul, Sub},
};

fn float_close(left: f64, right: f64) {
    let diff = (left - right).abs();
    assert!(
        diff / left.abs() < RELRATIO,
        "\n  left = {}\n  right = {}\n  rel = {}\n",
        left,
        right,
        diff / left.abs()
    );
    assert!(
        diff / right.abs() < RELRATIO,
        "\n  left = {}\n  right = {}\n  rel = {}\n",
        left,
        right,
        diff / left.abs()
    );
}

#[test]
fn test_scalar1() {
    let sv = 2.4;
    let s: Ad<1> = Ad::active_scalar(sv);
    let g = s.powi(3).grad()[(0, 0)];
    assert_abs_diff_eq!(g, 3.0 * sv * sv, epsilon = EPS);

    let sv = -3.42;
    let s: Ad<1> = Ad::active_scalar(sv);
    let g = s.sin().mul(&s).grad()[(0, 0)];
    let h = s.sin().mul(&s).hess()[(0, 0)];
    assert_abs_diff_eq!(g, sv * sv.cos() + sv.sin(), epsilon = EPS);
    assert_abs_diff_eq!(h, 2.0 * sv.cos() - sv * sv.sin(), epsilon = EPS);

    let sv = 1.4623;
    let s: Ad<1> = Ad::active_scalar(sv);
    let expr = s
        .cosh()
        .mul(&s.sinh().mul(&(Ad::inactive_scalar(1.245) / s.powi(-2))))
        .add(&s.tanh());
    let g = expr.grad()[(0, 0)];
    assert_abs_diff_eq!(g, grad_0(sv), epsilon = EPS);
    let h = expr.hess()[(0, 0)];
    assert_abs_diff_eq!(h, hess_0(sv), epsilon = EPS);

    let sv = 31.8;
    let s: Ad<1> = Ad::active_scalar(sv);
    let expr = s
        .tan()
        .mul(&s.asinh())
        .mul(&s)
        .sub(&s.powf(1.3).mul(&s.cos()))
        .add(&s.sqrt());
    let g = expr.grad()[(0, 0)];
    assert_abs_diff_eq!(g, grad_1(sv), epsilon = EPS);
    let h = expr.hess()[(0, 0)];
    assert_abs_diff_eq!(h, hess_1(sv), epsilon = EPS);
}

#[test]
fn test_scalar2() {
    let mut rng = thread_rng();

    for test_it in 0..100 {
        println!("Testing round {test_it}...");

        let sv = rng.gen_range(0.0..21.4124);
        let s: Ad<1> = Ad::active_scalar(sv);
        let g = s.sqrt().grad()[(0, 0)];
        assert_abs_diff_eq!(g, 0.5 / sv.sqrt(), epsilon = EPS);

        let sv = rng.gen_range(0.0..50.235);
        let s: Ad<1> = Ad::active_scalar(sv);
        let res = s.ln().mul(&s) + s.ln();
        let g = res.grad()[(0, 0)];
        let h = res.hess()[(0, 0)];
        assert_abs_diff_eq!(g, sv.ln() + 1.0 + sv.recip(), epsilon = EPS);

        // Mind the domain of ln
        let sv = rng.gen_range(0.0..50.235);
        let s: Ad<1> = Ad::active_scalar(sv);
        let res = s.log10().mul(&s) + s.log2();
        let g = res.grad()[(0, 0)];
        let h = res.hess()[(0, 0)];
        assert_abs_diff_eq!(
            g,
            (sv.ln() + 1.0) / 10_f64.ln() + sv.recip() * 2_f64.ln().recip(),
            epsilon = EPS
        );

        let sv = rng.gen_range(-12.2..3.12);
        let s: Ad<1> = Ad::active_scalar(sv);
        let res = s.exp().mul(&s) + (-s).exp();
        let g = res.grad()[(0, 0)];
        let h = res.hess()[(0, 0)];
        assert_abs_diff_eq!(g, (sv + 1.0) * sv.exp() - (-sv).exp(), epsilon = EPS);

        let sv = rng.gen_range(-12.2..3.12);
        let s: Ad<1> = Ad::active_scalar(sv);
        let res = s.cos() * s.tan();
        let g = res.grad()[(0, 0)];
        let h = res.hess()[(0, 0)];
        assert_abs_diff_eq!(g, grad_costan(sv), epsilon = EPS);
        assert_abs_diff_eq!(h, hess_costan(sv), epsilon = BIG_EPS);

        // test alpha
        let sv = rng.gen_range(-1.0..1.0);
        let s: Ad<1> = Ad::active_scalar(sv);
        let res = s.asin() * s.tan() + &s * s.acos();
        let g = res.grad()[(0, 0)];
        let h = res.hess()[(0, 0)];
        assert_abs_diff_eq!(g, grad_alpha(sv), epsilon = EPS);
        assert_abs_diff_eq!(h, hess_alpha(sv), epsilon = BIG_EPS);

        // test beta
        let sv = rng.gen_range(-2.14514..4.919810);
        let s: Ad<1> = Ad::active_scalar(sv);
        let res = s.atan2(&s.recip()) * s.sinh() + &s * s.cosh().powi(-3);
        let g = res.grad()[(0, 0)];
        let h = res.hess()[(0, 0)];
        assert_abs_diff_eq!(g, grad_beta(sv), epsilon = EPS);
        assert_abs_diff_eq!(h, hess_beta(sv), epsilon = BIG_EPS);

        // test kappa
        let sv = rng.gen_range(1.0..114.514);
        let s: Ad<1> = Ad::active_scalar(sv);
        let res = s.atan2(&s.asinh()) * s.acosh() + &s * s.tanh().powi(-3) - s.recip().atanh();
        let g = res.grad()[(0, 0)];
        let h = res.hess()[(0, 0)];
        assert_abs_diff_eq!(g, grad_kappa(sv), epsilon = EPS);
        assert_abs_diff_eq!(h, hess_kappa(sv), epsilon = BIG_EPS);
    }
}

#[test]
fn test_relative() {
    let sv = 0.2127;
    let s: Ad<1> = Ad::active_scalar(sv);
    let expr = s
        .tan()
        .mul(&s.asinh().mul(&s.atanh()))
        .mul(&s)
        .sub(&s.powf(1.3).mul(&s.cos()))
        .add(&s.sqrt())
        .sub(&s.div_value(1.441).add(&s.recip()))
        .sub(&(Ad::inactive_scalar(-6.235) / s));

    let g = expr.grad()[(0, 0)];
    float_close(g, grad_2(sv));
    let h = expr.hess()[(0, 0)];
    float_close(h, hess_2(sv));

    let sv = 0.8235;
    let s: Ad<1> = Ad::active_scalar(sv);
    let expr = s
        .tan()
        .mul(&s.asinh().square())
        .mul(&s)
        .sub(&s.powf(1.3).mul(&s.cos()))
        .add(&s.sqrt())
        .sub(&s.div_value(1.441).add(&s.recip()))
        .sub(&(Ad::inactive_scalar(-6.235) / s.clone()).add(&s.powi(3).sinh()))
        .add(&(s.powf(-1.24).div(&s.div_value(12.4).abs())))
        .abs();

    let g = expr.grad()[(0, 0)];
    float_close(g, grad_3(sv));
}
