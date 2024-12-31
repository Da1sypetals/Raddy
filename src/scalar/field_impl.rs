#![allow(unused)]

use crate::Ad;
use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use na::{ComplexField, Field, RealField, SimdValue};
use num_traits::FromPrimitive;
use simba::scalar::SubsetOf;
use std::f64::consts::LN_2;

// ################################################
// ################## Unexamined ##################
// ################################################

impl<const N: usize> AbsDiffEq for Ad<N> {
    type Epsilon = Self;

    fn default_epsilon() -> Self::Epsilon {
        todo!()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        todo!()
    }
}

impl<const N: usize> UlpsEq for Ad<N> {
    fn default_max_ulps() -> u32 {
        todo!()
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        todo!()
    }
}

impl<const N: usize> RelativeEq for Ad<N> {
    fn default_max_relative() -> Self::Epsilon {
        todo!()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        todo!()
    }
}

impl<const N: usize> Field for Ad<N> {}

impl<const N: usize> SimdValue for Ad<N> {
    const LANES: usize = 1;

    type Element = Self;

    type SimdBool = bool;

    fn splat(val: Self::Element) -> Self {
        todo!()
    }

    fn extract(&self, i: usize) -> Self::Element {
        todo!()
    }

    unsafe fn extract_unchecked(&self, i: usize) -> Self::Element {
        todo!()
    }

    fn replace(&mut self, i: usize, val: Self::Element) {
        todo!()
    }

    unsafe fn replace_unchecked(&mut self, i: usize, val: Self::Element) {
        todo!()
    }

    fn select(self, cond: Self::SimdBool, other: Self) -> Self {
        todo!()
    }
}

impl<const N: usize> FromPrimitive for Ad<N> {
    fn from_i64(n: i64) -> Option<Self> {
        todo!()
    }

    fn from_u64(n: u64) -> Option<Self> {
        todo!()
    }
}

impl<const N: usize> SubsetOf<Ad<N>> for Ad<N> {
    fn to_superset(&self) -> Ad<N> {
        todo!()
    }

    fn from_superset_unchecked(element: &Ad<N>) -> Self {
        todo!()
    }

    fn is_in_subset(element: &Ad<N>) -> bool {
        todo!()
    }
}

impl<const N: usize> SubsetOf<Ad<N>> for f64 {
    fn to_superset(&self) -> Ad<N> {
        todo!()
    }

    fn from_superset_unchecked(element: &Ad<N>) -> Self {
        todo!()
    }

    fn is_in_subset(element: &Ad<N>) -> bool {
        todo!()
    }
}
impl<const N: usize> SubsetOf<Ad<N>> for f32 {
    fn to_superset(&self) -> Ad<N> {
        todo!()
    }

    fn from_superset_unchecked(element: &Ad<N>) -> Self {
        todo!()
    }

    fn is_in_subset(element: &Ad<N>) -> bool {
        todo!()
    }
}

impl<const N: usize> RealField for Ad<N> {
    fn is_sign_positive(&self) -> bool {
        todo!()
    }

    fn is_sign_negative(&self) -> bool {
        todo!()
    }

    fn copysign(self, sign: Self) -> Self {
        todo!()
    }

    fn max(self, other: Self) -> Self {
        todo!()
    }

    fn min(self, other: Self) -> Self {
        todo!()
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        todo!()
    }

    fn atan2(self, other: Self) -> Self {
        todo!()
    }

    fn min_value() -> Option<Self> {
        todo!()
    }

    fn max_value() -> Option<Self> {
        todo!()
    }

    fn pi() -> Self {
        todo!()
    }

    fn two_pi() -> Self {
        todo!()
    }

    fn frac_pi_2() -> Self {
        todo!()
    }

    fn frac_pi_3() -> Self {
        todo!()
    }

    fn frac_pi_4() -> Self {
        todo!()
    }

    fn frac_pi_6() -> Self {
        todo!()
    }

    fn frac_pi_8() -> Self {
        todo!()
    }

    fn frac_1_pi() -> Self {
        todo!()
    }

    fn frac_2_pi() -> Self {
        todo!()
    }

    fn frac_2_sqrt_pi() -> Self {
        todo!()
    }

    fn e() -> Self {
        todo!()
    }

    fn log2_e() -> Self {
        todo!()
    }

    fn log10_e() -> Self {
        todo!()
    }

    fn ln_2() -> Self {
        todo!()
    }

    fn ln_10() -> Self {
        todo!()
    }
}

// ################################################
// ################### Examined ###################
// ################################################

impl<const N: usize> ComplexField for Ad<N> {
    type RealField = Ad<N>;

    #[doc = r" Builds a pure-real complex number from the given value."]
    fn from_real(re: Self::RealField) -> Self {
        re
    }

    #[doc = r" The real part of this complex number."]
    fn real(self) -> Self::RealField {
        self
    }

    #[doc = r" The imaginary part of this complex number."]
    fn imaginary(self) -> Self::RealField {
        unimplemented!("This is a real type");
    }

    #[doc = r" The modulus of this complex number."]
    fn modulus(self) -> Self::RealField {
        self.abs()
    }

    #[doc = r" The squared modulus of this complex number."]
    fn modulus_squared(self) -> Self::RealField {
        self.square()
    }

    #[doc = r" The argument of this complex number."]
    /// This should be zero with no grad w.r.t. self, but the use of this method is itself a bug.
    fn argument(self) -> Self::RealField {
        unimplemented!("This should not be used");
    }

    #[doc = r" The sum of the absolute value of this complex number's real and imaginary part."]
    fn norm1(self) -> Self::RealField {
        self.abs()
    }

    #[doc = r" Multiplies this complex number by `factor`."]
    fn scale(self, factor: Self::RealField) -> Self {
        factor * self
    }

    #[doc = r" Divides this complex number by `factor`."]
    fn unscale(self, factor: Self::RealField) -> Self {
        self / factor
    }

    fn floor(self) -> Self {
        unimplemented!("Floor is not differentiable!");
    }

    fn ceil(self) -> Self {
        unimplemented!("Ceil is not differentiable!");
    }

    fn round(self) -> Self {
        unimplemented!("Round is not differentiable!");
    }

    fn trunc(self) -> Self {
        unimplemented!("Trunc is not differentiable!");
    }

    fn fract(self) -> Self {
        unimplemented!("Fract is not differentiable!");
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        a * self + b
    }

    #[doc = r" The absolute value of this complex number: `self / self.signum()`."]
    #[doc = r""]
    #[doc = r" This is equivalent to `self.modulus()`."]
    fn abs(self) -> Self::RealField {
        let mut res = Self::_zeroed();
        res.value = self.value.abs();
        let sign = if self.value >= 0.0 { 1.0 } else { -1.0 };
        res.grad = sign * self.grad;
        res.hess = sign * self.hess;

        res
    }

    #[doc = r" Computes (self.conjugate() * self + other.conjugate() * other).sqrt()"]
    fn hypot(self, other: Self) -> Self::RealField {
        (&self * &self + &other * &other).sqrt()
    }

    fn recip(self) -> Self {
        Ad::inactive_scalar(1.0) / self
    }

    /// Real number has itself as conjugate
    fn conjugate(self) -> Self {
        self
    }

    fn sin(self) -> Self {
        let sin_val = self.value.sin();
        let cos_val = self.value.cos();

        Self::chain(sin_val, cos_val, -sin_val, &self)
    }

    fn cos(self) -> Self {
        let cos_val = self.value.cos();
        let sin_val = self.value.sin();

        Self::chain(cos_val, -sin_val, -cos_val, &self)
    }

    fn sin_cos(self) -> (Self, Self) {
        // (self.sin(), self.cos())
        todo!()
    }

    fn tan(self) -> Self {
        let cos_val = self.value.cos();
        let cos_sq = cos_val * cos_val;

        Self::chain(
            self.value.tan(),
            1.0 / cos_sq,
            2.0 * self.value.sin() / (cos_sq * cos_val),
            &self,
        )
    }

    fn asin(self) -> Self {
        if self.value < -1.0 || self.value > 1.0 {
            panic!("Asin out of domain!");
        }
        let s = 1.0 - self.value * self.value;
        let s_sqrt = s.sqrt();

        Self::chain(
            self.value.asin(),
            1.0 / s_sqrt,
            self.value / (s * s_sqrt),
            &self,
        )
    }

    fn acos(self) -> Self {
        if self.value < -1.0 || self.value > 1.0 {
            panic!("Acos out of domain!");
        }
        let s = 1.0 - self.value * self.value;
        let s_sqrt = s.sqrt();

        Self::chain(
            self.value.acos(),
            -1.0 / s_sqrt,
            -self.value / (s * s_sqrt),
            &self,
        )
    }

    fn atan(self) -> Self {
        let s = self.value * self.value + 1.0;

        Self::chain(
            self.value.atan(),
            1.0 / s,
            -2.0 * self.value / (s * s),
            &self,
        )
    }

    fn sinh(self) -> Self {
        let sinh_val = self.value.sinh();
        let cosh_val = self.value.cosh();

        Self::chain(sinh_val, cosh_val, sinh_val, &self)
    }

    fn cosh(self) -> Self {
        let sinh_val = self.value.sinh();
        let cosh_val = self.value.cosh();

        Self::chain(cosh_val, sinh_val, cosh_val, &self)
    }

    fn tanh(self) -> Self {
        let cosh_val = self.value.cosh();
        let cosh_sq = cosh_val * cosh_val;

        Self::chain(
            self.value.tanh(),
            1.0 / cosh_sq,
            -2.0 * self.value.sinh() / (cosh_sq * cosh_val),
            &self,
        )
    }

    fn asinh(self) -> Self {
        let s = self.value * self.value + 1.0;
        let s_sqrt = s.sqrt();

        Self::chain(
            self.value.asinh(),
            1.0 / s_sqrt,
            -self.value / (s * s_sqrt),
            &self,
        )
    }

    fn acosh(self) -> Self {
        if self.value < 1.0 {
            panic!("Acosh out of domain!");
        }
        let sm = self.value - 1.0;
        let sp = self.value + 1.0;
        let prod = (sm * sp).sqrt();

        Self::chain(
            self.value.acosh(),
            1.0 / prod,
            -self.value / (prod * sm * sp),
            &self,
        )
    }

    fn atanh(self) -> Self {
        if self.value <= -1.0 || self.value >= 1.0 {
            panic!("Atanh out of domain!");
        }
        let s = 1.0 - self.value * self.value;

        Self::chain(
            self.value.atanh(),
            1.0 / s,
            2.0 * self.value / (s * s),
            &self,
        )
    }

    fn log(self, base: Self::RealField) -> Self {
        unimplemented!("Differentiation w.r.t. base is not implemented...")
    }

    fn log2(self) -> Self {
        if self.value <= 0.0 {
            panic!("Log2 on non-positive value!");
        }
        let inv = 1.0 / self.value / std::f64::consts::LN_2;

        Self::chain(self.value.log2(), inv, -inv / self.value, &self)
    }

    fn log10(self) -> Self {
        if self.value <= 0.0 {
            panic!("Log10 on non-positive value!");
        }
        let inv = 1.0 / self.value / std::f64::consts::LN_10;

        Self::chain(self.value.log10(), inv, -inv / self.value, &self)
    }

    fn ln(self) -> Self {
        if self.value <= 0.0 {
            panic!("Ln on non-positive value!");
        }
        let inv = 1.0 / self.value;

        Self::chain(self.value.ln(), inv, -inv * inv, &self)
    }

    fn ln_1p(self) -> Self {
        (self + Self::inactive_scalar(1.0)).ln()
    }

    fn sqrt(self) -> Self {
        if self.value < -0.0 {
            // We don't want to mute this behavior or get NaN as this is fucking undebuggable.
            panic!("Sqrt on negative value!");
        }
        let f = self.value.sqrt();

        Self::chain(f, 0.5 / f, -0.25 / (f * self.value), &self)
    }

    fn exp(self) -> Self {
        let exp_val = self.value.exp();

        Self::chain(exp_val, exp_val, exp_val, &self)
    }

    fn exp2(self) -> Self {
        let exp_val = self.value.exp2();

        Self::chain(exp_val, exp_val * LN_2, exp_val * LN_2 * LN_2, &self)
    }

    fn exp_m1(self) -> Self {
        (self - Self::inactive_scalar(1.0)).exp()
    }

    fn powi(self, exponent: i32) -> Self {
        if self.value.abs() == 0.0 && exponent == 0 {
            // We don't want to mute this behavior or get NaN as this is fucking undebuggable.
            panic!("0.pow(0) is undefined!");
        }

        let f2 = self.value.powi(exponent - 2);
        let f1 = f2 * self.value;
        let f = f1 * self.value;

        // exponent in float
        let ef = exponent as f64;

        Self::chain(f, ef * f1, ef * (ef - 1.0) * f2, &self)
    }

    fn powf(self, n: Self::RealField) -> Self {
        unimplemented!("Differentiation w.r.t. power it not supported");
    }

    fn powc(self, n: Self) -> Self {
        unimplemented!("Differentiation w.r.t. complex power it not supported");
    }

    fn cbrt(self) -> Self {
        let f = self.value.cbrt();

        let d = 1.0 / (3.0 * f * f);
        let dd = -2.0 / (9.0 * f * f * f * self.value);

        Self::chain(f, d, dd, &self)
    }

    fn is_finite(&self) -> bool {
        self.value.is_finite()
            && self.grad.as_slice().into_iter().all(|x| x.is_finite())
            && self.hess.as_slice().into_iter().all(|x| x.is_finite())
    }

    fn try_sqrt(self) -> Option<Self> {
        if self.value < -0.0 {
            None
        } else {
            Some(self.sqrt())
        }
    }
}

// ################################################
// #################### Tests #####################
// ################################################

#[cfg(test)]
mod test_field_impl {
    use crate::{misc::symbolic_1::grad_det3, types::advec, Ad, GetValue};
    use approx::assert_abs_diff_eq;
    use na::U3;
    use rand::{thread_rng, Rng};

    const EPS: f64 = 1e-12;

    #[test]
    fn test_det() {
        const N: usize = 3;
        const NVEC: usize = N * N;
        let mut rng = thread_rng();
        let vals: Vec<_> = (0..NVEC).map(|_| rng.gen_range(-3.0..3.0)).collect();
        let matvec: advec<9, 9> = Ad::vec(&vals);

        // Note that this reshape is ROW MAJOR, we shall transpose it
        let mat: na::SMatrix<Ad<NVEC>, 3, 3> = matvec.reshape_generic(U3, U3).transpose();

        let mat_val = mat.value();

        let det = mat.determinant();
        let gt_det = mat_val.determinant();

        let det_grad = det.grad();
        let gt_det_grad = grad_det3(
            mat_val[(0, 0)],
            mat_val[(0, 1)],
            mat_val[(0, 2)],
            mat_val[(1, 0)],
            mat_val[(1, 1)],
            mat_val[(1, 2)],
            mat_val[(2, 0)],
            mat_val[(2, 1)],
            mat_val[(2, 2)],
        );

        assert_eq!(det.value(), gt_det);

        let grad_diff = (det_grad - gt_det_grad).norm_squared();
        assert_abs_diff_eq!(grad_diff, 0.0, epsilon = EPS);
    }
}
