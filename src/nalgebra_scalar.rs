#![allow(unused)]
use crate::Variable;
use num_traits::{Num, One, Signed, Zero};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

impl<const N: usize> Display for Variable<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ad[{}]", self.value)
    }
}

impl<const N: usize> Neg for Variable<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut res = Self::_zeroed();
        res.value = -self.value;
        res.grad = -self.grad;
        res.hess = -self.hess;

        res
    }
}

impl<const N: usize> Add for Variable<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Self::_zeroed();
        res.value = self.value + rhs.value;
        res.grad = self.grad + rhs.grad;
        res.hess = self.hess + rhs.hess;

        res
    }
}

impl<const N: usize> AddAssign for Variable<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl<const N: usize> Sub for Variable<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = Self::_zeroed();
        res.value = self.value - rhs.value;
        res.grad = self.grad - rhs.grad;
        res.hess = self.hess - rhs.hess;

        res
    }
}

impl<const N: usize> SubAssign for Variable<N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}

impl<const N: usize> Mul for Variable<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Self::_zeroed();
        res.value = self.value * rhs.value;
        res.grad = self.grad * rhs.value + self.value * rhs.grad;
        res.hess = rhs.value * self.hess
            + self.value * rhs.hess
            + self.grad * rhs.grad.transpose()
            + rhs.grad * self.grad.transpose();

        res
    }
}

impl<const N: usize> MulAssign for Variable<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl<const N: usize> Div for Variable<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        if rhs.value.abs() == 0.0 {
            // We don't want to mute this behavior or get NaN as this is fucking undebuggable.
            panic!("Division By Zero!");
        }

        let mut res = Self::_zeroed();
        res.value = self.value / rhs.value;
        res.grad = (rhs.value * self.grad - self.value * rhs.grad) / (rhs.value * rhs.value);
        res.hess = (self.hess
            - res.grad * rhs.grad.transpose()
            - rhs.grad * res.grad.transpose()
            - res.value * rhs.hess)
            / rhs.value;

        res
    }
}

impl<const N: usize> DivAssign for Variable<N> {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone() / rhs;
    }
}

impl<const N: usize> Rem for Variable<N> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        unimplemented!()
    }
}

impl<const N: usize> RemAssign for Variable<N> {
    fn rem_assign(&mut self, rhs: Self) {
        unimplemented!()
    }
}

impl<const N: usize> Zero for Variable<N> {
    fn zero() -> Self {
        Variable::_zeroed()
    }

    fn is_zero(&self) -> bool {
        self.value.abs() == 0.0
            && self
                .grad
                .abs()
                .as_slice()
                .into_iter()
                .all(|&x| x.abs() == 0.0)
            && self
                .hess
                .abs()
                .as_slice()
                .into_iter()
                .all(|&x| x.abs() == 0.0)
    }
}

impl<const N: usize> One for Variable<N> {
    fn one() -> Self {
        let mut res = Variable::_zeroed();
        res.value = 1.0;
        res
    }
}

impl<const N: usize> Num for Variable<N> {
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        unimplemented!()
    }
}

impl<const N: usize> Signed for Variable<N> {
    fn abs(&self) -> Self {
        let mut res = Self::_zeroed();
        res.value = self.value.abs();
        let sign = if self.value >= 0.0 { 1.0 } else { -1.0 };
        res.grad = sign * self.grad;
        res.hess = sign * self.hess;

        res
    }

    fn abs_sub(&self, other: &Self) -> Self {
        unimplemented!()
    }

    fn signum(&self) -> Self {
        unimplemented!()
    }

    fn is_positive(&self) -> bool {
        self.value > 0.0
    }

    fn is_negative(&self) -> bool {
        self.value < -0.0
    }
}
