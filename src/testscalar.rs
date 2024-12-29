use nalgebra::{Matrix3, SMatrix};
use num_traits::{Num, One, Signed, Zero};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

#[derive(Debug, PartialEq, Clone)]
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

    println!("{}\n{}\n{}", r, n, x);
}
