use crate::{commutative::Commutative, Variable};
use approx::assert_abs_diff_eq;

const EPS: f64 = 1e-12;
const RELRATIO: f64 = 6e-4;

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

// code generated by symars (sympy)
pub fn grad_1(s: f64) -> f64 {
    (((0.50000000000000000000_f64) * ((s).powf(-0.50000000000000000000_f64)))
        + (((s).powf(1.30000000000000_f64)) * ((s).sin()))
        + (((s).asinh()) * ((s).tan()))
        + ((-1.30000000000000_f64) * ((s).powf(0.300000000000000_f64)) * ((s).cos()))
        + ((s)
            * (((1.0000000000000000000_f64) + ((s).powi(2))).powf(-0.50000000000000000000_f64))
            * ((s).tan()))
        + ((s) * ((1.0000000000000000000_f64) + (((s).tan()).powi(2))) * ((s).asinh())))
}

pub fn hess_1(s: f64) -> f64 {
    let __intermediate_result_8 = ((1.0000000000000000000_f64) + ((s).powi(2)));
    let __intermediate_result_10 = (s).tan();
    let __intermediate_result_13 = ((s).tan()).powi(2);

    (((-0.25000000000000000000_f64) * ((s).powf(-1.5000000000000000000_f64)))
        + (((s).powf(1.30000000000000_f64)) * ((s).cos()))
        + ((2.0000000000000000000_f64)
            * ((__intermediate_result_8).powf(-0.50000000000000000000_f64))
            * (__intermediate_result_10))
        + ((2.0000000000000000000_f64)
            * ((1.0000000000000000000_f64) + (__intermediate_result_13))
            * ((s).asinh()))
        + ((2.60000000000000_f64) * ((s).powf(0.300000000000000_f64)) * ((s).sin()))
        + ((-0.390000000000000_f64) * ((s).powf(-0.700000000000000_f64)) * ((s).cos()))
        + (-(((s).powi(2))
            * ((__intermediate_result_8).powf(-1.5000000000000000000_f64))
            * (__intermediate_result_10)))
        + ((2.0000000000000000000_f64)
            * (s)
            * ((__intermediate_result_8).powf(-0.50000000000000000000_f64))
            * ((1.0000000000000000000_f64) + (__intermediate_result_13)))
        + ((s)
            * ((2.0000000000000000000_f64)
                + ((2.0000000000000000000_f64) * (__intermediate_result_13)))
            * ((s).asinh())
            * (__intermediate_result_10)))
}

pub fn grad_2(s: f64) -> f64 {
    let __intermediate_result_12 = (s).powi(2);
    let __intermediate_result_15 = (s).atanh();
    let __intermediate_result_16 = (s).tan();
    let __intermediate_result_21 = (s).asinh();

    ((-0.693962526023595_f64)
        + ((0.50000000000000000000_f64) * ((s).powf(-0.50000000000000000000_f64)))
        + ((-5.23500000000000_f64) * ((s).powi(-2)))
        + (((s).powf(1.30000000000000_f64)) * ((s).sin()))
        + ((-1.30000000000000_f64) * ((s).powf(0.300000000000000_f64)) * ((s).cos()))
        + ((__intermediate_result_12)
            * (((1.0000000000000000000_f64) + (__intermediate_result_12))
                .powf(-0.50000000000000000000_f64))
            * (__intermediate_result_15)
            * (__intermediate_result_16))
        + ((__intermediate_result_12)
            * (((1.0000000000000000000_f64) + (-(__intermediate_result_12))).recip())
            * (__intermediate_result_21)
            * (__intermediate_result_16))
        + ((__intermediate_result_12)
            * ((1.0000000000000000000_f64) + ((__intermediate_result_16).powi(2)))
            * (__intermediate_result_21)
            * (__intermediate_result_15))
        + ((2.0000000000000000000_f64)
            * (s)
            * (__intermediate_result_21)
            * (__intermediate_result_15)
            * (__intermediate_result_16)))
}

pub fn grad_3(s: f64) -> f64 {
    let __intermediate_result_9 = (s).powf(1.30000000000000_f64);
    let __intermediate_result_12 = (s).powi(2);
    let __intermediate_result_14 = (s).powi(3);
    let __intermediate_result_17 = (s).cos();
    let __intermediate_result_19 = ((s).asinh()).powi(2);
    let __intermediate_result_23 = (s).tan();

    (((-0.693962526023595_f64)
        + ((0.50000000000000000000_f64) * ((s).powf(-0.50000000000000000000_f64)))
        + ((-27.7760000000000_f64) * ((s).powf(-3.24000000000000_f64)))
        + ((-5.23500000000000_f64) * ((s).powi(-2)))
        + ((__intermediate_result_9) * ((s).sin()))
        + ((-3.0000000000000000000_f64)
            * (__intermediate_result_12)
            * ((__intermediate_result_14).cosh()))
        + ((-1.30000000000000_f64)
            * ((s).powf(0.300000000000000_f64))
            * (__intermediate_result_17))
        + ((__intermediate_result_12)
            * (__intermediate_result_19)
            * ((1.0000000000000000000_f64) + ((__intermediate_result_23).powi(2))))
        + ((2.0000000000000000000_f64)
            * (s)
            * (__intermediate_result_19)
            * (__intermediate_result_23))
        + ((2.0000000000000000000_f64)
            * (__intermediate_result_12)
            * (((1.0000000000000000000_f64) + (__intermediate_result_12))
                .powf(-0.50000000000000000000_f64))
            * ((s).asinh())
            * (__intermediate_result_23)))
        * (if (((s).sqrt())
            + (-((__intermediate_result_14).sinh()))
            + ((5.23500000000000_f64) * ((s).recip()))
            + ((12.4000000000000_f64) * ((s).powf(-2.24000000000000_f64)))
            + ((-0.693962526023595_f64) * (s))
            + (-((__intermediate_result_9) * (__intermediate_result_17)))
            + ((__intermediate_result_12)
                * (__intermediate_result_19)
                * (__intermediate_result_23)))
            .abs()
            == 0.0_f64
        {
            (((s).sqrt())
                + (-((__intermediate_result_14).sinh()))
                + ((5.23500000000000_f64) * ((s).recip()))
                + ((12.4000000000000_f64) * ((s).powf(-2.24000000000000_f64)))
                + ((-0.693962526023595_f64) * (s))
                + (-((__intermediate_result_9) * (__intermediate_result_17)))
                + ((__intermediate_result_12)
                    * (__intermediate_result_19)
                    * (__intermediate_result_23)))
        } else {
            (((s).sqrt())
                + (-((__intermediate_result_14).sinh()))
                + ((5.23500000000000_f64) * ((s).recip()))
                + ((12.4000000000000_f64) * ((s).powf(-2.24000000000000_f64)))
                + ((-0.693962526023595_f64) * (s))
                + (-((__intermediate_result_9) * (__intermediate_result_17)))
                + ((__intermediate_result_12)
                    * (__intermediate_result_19)
                    * (__intermediate_result_23)))
                .signum()
        }))
}
#[test]
fn test_scalar() {
    let sv = 2.4;
    let s: Variable<1> = Variable::active_uni(2.4);
    let g = s.powi(3).grad()[(0, 0)];
    assert_abs_diff_eq!(g, 3.0 * sv * sv, epsilon = EPS);

    let sv = -3.42;
    let s: Variable<1> = Variable::active_uni(sv);
    let g = s.sin().mul(&s).grad()[(0, 0)];
    let h = s.sin().mul(&s).hess()[(0, 0)];
    assert_abs_diff_eq!(g, sv * sv.cos() + sv.sin(), epsilon = EPS);
    assert_abs_diff_eq!(h, 2.0 * sv.cos() - sv * sv.sin(), epsilon = EPS);

    let sv = -3.42;
    let s: Variable<1> = Variable::active_uni(sv);
    let g = s.sin().mul(&s).grad()[(0, 0)];
    let h = s.sin().mul(&s).hess()[(0, 0)];
    assert_abs_diff_eq!(g, sv * sv.cos() + sv.sin(), epsilon = EPS);
    assert_abs_diff_eq!(h, 2.0 * sv.cos() - sv * sv.sin(), epsilon = EPS);

    let sv = 31.8;
    let s: Variable<1> = Variable::active_uni(sv);
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

    let sv = 0.2127;
    let s: Variable<1> = Variable::active_uni(sv);
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
    // let h = expr.hess()[(0, 0)];
    // assert_abs_diff_eq!(h, hess_1(sv), epsilon = EPS);

    let sv = 0.8235;
    let s: Variable<1> = Variable::active_uni(sv);
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
