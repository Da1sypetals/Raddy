#![allow(unused)]

use crate::{
    norms::AdMatrixOps,
    scalar::commutative::Commutative,
    test::{
        symbolic::{grad_0, grad_1, grad_2, grad_3, hess_0, hess_1, hess_2},
        EPS, RELRATIO,
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
fn test_scalar() {
    let sv = 2.4;
    let s: Ad<1> = Ad::active_scalar(2.4);
    let g = s.powi(3).grad()[(0, 0)];
    assert_abs_diff_eq!(g, 3.0 * sv * sv, epsilon = EPS);

    let sv = -3.42;
    let s: Ad<1> = Ad::active_scalar(sv);
    let g = s.sin().mul(&s).grad()[(0, 0)];
    let h = s.sin().mul(&s).hess()[(0, 0)];
    assert_abs_diff_eq!(g, sv * sv.cos() + sv.sin(), epsilon = EPS);
    assert_abs_diff_eq!(h, 2.0 * sv.cos() - sv * sv.sin(), epsilon = EPS);

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
        .mul(&s.sinh().mul(&1.245.div_var(&s.powi(-2))))
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
        .sub(&(-6.235).div_var(&s));

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
        .sub(&(-6.235).div_var(&s).add(&s.powi(3).sinh()))
        .add(&(s.powf(-1.24).div(&s.div_value(12.4).abs())))
        .abs();

    let g = expr.grad()[(0, 0)];
    float_close(g, grad_3(sv));
}
