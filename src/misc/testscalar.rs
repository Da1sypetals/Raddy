#![allow(unused)]
// For test use. Not functional code.

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use na::{ComplexField, Field, RealField, SimdValue};
use na::{Matrix3, SMatrix};
use num_traits::{FromPrimitive, Num, One, Signed, Zero};
use simba::scalar::SubsetOf;
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct Val {}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Val Object]")
    }
}

impl Add for Val {
    type Output = Val;

    fn add(self, rhs: Self) -> Self::Output {
        Val {}
    }
}

impl Add<f64> for Val {
    type Output = Val;

    fn add(self, rhs: f64) -> Self::Output {
        Val {}
    }
}

impl AddAssign for Val {
    fn add_assign(&mut self, rhs: Self) {
        *self = Val {}
    }
}

impl Mul for Val {
    type Output = Val;

    fn mul(self, rhs: Self) -> Self::Output {
        Val {}
    }
}

impl MulAssign for Val {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Val {}
    }
}

impl One for Val {
    fn one() -> Self {
        Val {}
    }
}

impl Zero for Val {
    fn zero() -> Self {
        Val {}
    }

    fn is_zero(&self) -> bool {
        true
    }
}

impl Neg for Val {
    type Output = Val;

    fn neg(self) -> Self::Output {
        Val {}
    }
}

impl Sub for Val {
    type Output = Val;

    fn sub(self, rhs: Self) -> Self::Output {
        Val {}
    }
}

impl SubAssign for Val {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Val {}
    }
}

impl Div for Val {
    type Output = Val;

    fn div(self, rhs: Self) -> Self::Output {
        Val {}
    }
}

impl DivAssign for Val {
    fn div_assign(&mut self, rhs: Self) {
        *self = Val {}
    }
}

impl Rem for Val {
    type Output = Val;

    fn rem(self, rhs: Self) -> Self::Output {
        Val {}
    }
}

impl RemAssign for Val {
    fn rem_assign(&mut self, rhs: Self) {
        *self = Val {}
    }
}

impl Num for Val {
    type FromStrRadixErr = f64;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        // converts from string to integer. useless in our case.
        unimplemented!()
    }
}

impl Signed for Val {
    fn abs(&self) -> Self {
        Val {}
    }

    fn abs_sub(&self, other: &Self) -> Self {
        Val {}
    }

    fn signum(&self) -> Self {
        Val {}
    }

    fn is_positive(&self) -> bool {
        true
    }

    fn is_negative(&self) -> bool {
        false
    }
}

impl AbsDiffEq for Val {
    type Epsilon = Val;

    fn default_epsilon() -> Self::Epsilon {
        todo!()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        todo!()
    }
}

impl UlpsEq for Val {
    fn default_max_ulps() -> u32 {
        todo!()
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        todo!()
    }
}

impl RelativeEq for Val {
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

impl Field for Val {}

impl SimdValue for Val {
    const LANES: usize = 0;

    type Element = Val;

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

impl FromPrimitive for Val {
    fn from_i64(n: i64) -> Option<Self> {
        todo!()
    }

    fn from_u64(n: u64) -> Option<Self> {
        todo!()
    }
}

impl SubsetOf<Val> for Val {
    fn to_superset(&self) -> Val {
        todo!()
    }

    fn from_superset_unchecked(element: &Val) -> Self {
        todo!()
    }

    fn is_in_subset(element: &Val) -> bool {
        todo!()
    }
}

impl SubsetOf<Val> for f64 {
    fn to_superset(&self) -> Val {
        todo!()
    }

    fn from_superset_unchecked(element: &Val) -> Self {
        todo!()
    }

    fn is_in_subset(element: &Val) -> bool {
        todo!()
    }
}

impl SubsetOf<Val> for f32 {
    fn to_superset(&self) -> Val {
        todo!()
    }

    fn from_superset_unchecked(element: &Val) -> Self {
        todo!()
    }

    fn is_in_subset(element: &Val) -> bool {
        todo!()
    }
}

impl RealField for Val {
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

impl ComplexField for Val {
    type RealField = Val;

    #[doc = r" Builds a pure-real complex number from the given value."]
    fn from_real(re: Self::RealField) -> Self {
        todo!()
    }

    #[doc = r" The real part of this complex number."]
    fn real(self) -> Self::RealField {
        todo!()
    }

    #[doc = r" The imaginary part of this complex number."]
    fn imaginary(self) -> Self::RealField {
        todo!()
    }

    #[doc = r" The modulus of this complex number."]
    fn modulus(self) -> Self::RealField {
        todo!()
    }

    #[doc = r" The squared modulus of this complex number."]
    fn modulus_squared(self) -> Self::RealField {
        todo!()
    }

    #[doc = r" The argument of this complex number."]
    fn argument(self) -> Self::RealField {
        todo!()
    }

    #[doc = r" The sum of the absolute value of this complex number's real and imaginary part."]
    fn norm1(self) -> Self::RealField {
        todo!()
    }

    #[doc = r" Multiplies this complex number by `factor`."]
    fn scale(self, factor: Self::RealField) -> Self {
        todo!()
    }

    #[doc = r" Divides this complex number by `factor`."]
    fn unscale(self, factor: Self::RealField) -> Self {
        todo!()
    }

    fn floor(self) -> Self {
        todo!()
    }

    fn ceil(self) -> Self {
        todo!()
    }

    fn round(self) -> Self {
        todo!()
    }

    fn trunc(self) -> Self {
        todo!()
    }

    fn fract(self) -> Self {
        todo!()
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        todo!()
    }

    #[doc = r" The absolute value of this complex number: `self / self.signum()`."]
    #[doc = r""]
    #[doc = r" This is equivalent to `self.modulus()`."]
    fn abs(self) -> Self::RealField {
        todo!()
    }

    #[doc = r" Computes (self.conjugate() * self + other.conjugate() * other).sqrt()"]
    fn hypot(self, other: Self) -> Self::RealField {
        todo!()
    }

    fn recip(self) -> Self {
        todo!()
    }

    fn conjugate(self) -> Self {
        todo!()
    }

    fn sin(self) -> Self {
        todo!()
    }

    fn cos(self) -> Self {
        todo!()
    }

    fn sin_cos(self) -> (Self, Self) {
        todo!()
    }

    fn tan(self) -> Self {
        todo!()
    }

    fn asin(self) -> Self {
        todo!()
    }

    fn acos(self) -> Self {
        todo!()
    }

    fn atan(self) -> Self {
        todo!()
    }

    fn sinh(self) -> Self {
        todo!()
    }

    fn cosh(self) -> Self {
        todo!()
    }

    fn tanh(self) -> Self {
        todo!()
    }

    fn asinh(self) -> Self {
        todo!()
    }

    fn acosh(self) -> Self {
        todo!()
    }

    fn atanh(self) -> Self {
        todo!()
    }

    fn log(self, base: Self::RealField) -> Self {
        todo!()
    }

    fn log2(self) -> Self {
        todo!()
    }

    fn log10(self) -> Self {
        todo!()
    }

    fn ln(self) -> Self {
        todo!()
    }

    fn ln_1p(self) -> Self {
        todo!()
    }

    fn sqrt(self) -> Self {
        todo!()
    }

    fn exp(self) -> Self {
        todo!()
    }

    fn exp2(self) -> Self {
        todo!()
    }

    fn exp_m1(self) -> Self {
        todo!()
    }

    fn powi(self, n: i32) -> Self {
        todo!()
    }

    fn powf(self, n: Self::RealField) -> Self {
        todo!()
    }

    fn powc(self, n: Self) -> Self {
        todo!()
    }

    fn cbrt(self) -> Self {
        todo!()
    }

    fn is_finite(&self) -> bool {
        todo!()
    }

    fn try_sqrt(self) -> Option<Self> {
        todo!()
    }
}

#[test]
fn example() {
    let fm = SMatrix::<f64, 3, 3>::zeros();
    let mat: SMatrix<Val, 3, 3> = SMatrix::from_row_slice(&[
        Val {},
        Val {},
        Val {},
        Val {},
        Val {},
        Val {},
        Val {},
        Val {},
        Val {},
    ]);

    let r = &mat * &mat;
    let n = mat.abs();
    let x = mat.transpose();
    // let m = Val {} * mat;
    let m = 2.0 * Matrix3::identity();

    let det = mat.determinant();

    dbg!(det);

    println!("{}\n{}\n{}", r, n, x);
}
