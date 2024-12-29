#![allow(unused)]
use crate::Ad;
use nalgebra::SMatrix;
use num_traits::{Num, One, Signed, Zero};
use std::{
    fmt::Display,
    mem::zeroed,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

impl<const N: usize> Display for Ad<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ad[{}]", self.value)
    }
}

impl<const N: usize> Zero for Ad<N> {
    fn zero() -> Self {
        Ad::_zeroed()
    }

    fn is_zero(&self) -> bool {
        self.value.abs() == 0.0
            && self.grad.as_slice().into_iter().all(|&x| x.abs() == 0.0)
            && self.hess.as_slice().into_iter().all(|&x| x.abs() == 0.0)
    }
}

impl<const N: usize> One for Ad<N> {
    fn one() -> Self {
        let mut res = Ad::_zeroed();
        res.value = 1.0;
        res
    }
}

impl<const N: usize> Num for Ad<N> {
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        unimplemented!()
    }
}

impl<const N: usize> Signed for Ad<N> {
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

/*


// impl<const N: usize> Neg for Ad<N> {
//     type Output = Ad<N>;

//     fn neg(self) -> Self::Output {
//         let mut res = Ad::<N>::_zeroed();
//         res.value = -self.value;
//         res.grad = -self.grad;
//         res.hess = -self.hess;

//         res
//     }
// }

// impl<const N: usize> Neg for &Ad<N> {
//     type Output = Ad<N>;

//     fn neg(self) -> Self::Output {
//         let mut res = Ad::<N>::_zeroed();
//         res.value = -self.value;
//         res.grad = -self.grad;
//         res.hess = -self.hess;

//         res
//     }
// }

// impl<const N: usize> Add for Ad<N> {
//     type Output = Ad<N>;

//     fn add(self, rhs: Self) -> Self::Output {
//         let mut res = Ad::<N>::_zeroed();
//         res.value = self.value + rhs.value;
//         res.grad = self.grad + rhs.grad;
//         res.hess = self.hess + rhs.hess;

//         res
//     }
// }

// impl<const N: usize> Add<&Ad<N>> for Ad<N> {
//     type Output = Ad<N>;

//     fn add(self, rhs: &Ad<N>) -> Self::Output {
//         let mut res = Ad::<N>::_zeroed();
//         res.value = self.value + rhs.value;
//         res.grad = self.grad + rhs.grad;
//         res.hess = self.hess + rhs.hess;

//         res
//     }
// }

// impl<const N: usize> Add<Ad<N>> for &Ad<N> {
//     type Output = Ad<N>;

//     fn add(self, rhs: Ad<N>) -> Self::Output {
//         let mut res = Ad::<N>::_zeroed();
//         res.value = self.value + rhs.value;
//         res.grad = self.grad + rhs.grad;
//         res.hess = self.hess + rhs.hess;

//         res
//     }
// }

// impl<const N: usize> Add<&Ad<N>> for &Ad<N> {
//     type Output = Ad<N>;

//     fn add(self, rhs: &Ad<N>) -> Self::Output {
//         let mut res = Ad::<N>::_zeroed();
//         res.value = self.value + rhs.value;
//         res.grad = self.grad + rhs.grad;
//         res.hess = self.hess + rhs.hess;

//         res
//     }
// }

// impl<const N: usize> AddAssign<Ad<N>> for Ad<N> {
//     fn add_assign(&mut self, rhs: Ad<N>) {
//         *self = self.clone() + rhs;
//     }
// }

// impl<const N: usize> AddAssign<&Ad<N>> for Ad<N> {
//     fn add_assign(&mut self, rhs: &Ad<N>) {
//         *self = self.clone() + rhs;
//     }
// }

// impl<const N: usize> Sub for Ad<N> {
//     type Output = Self;

//     fn sub(self, rhs: Self) -> Self::Output {
//         let mut res = Self::_zeroed();
//         res.value = self.value - rhs.value;
//         res.grad = self.grad - rhs.grad;
//         res.hess = self.hess - rhs.hess;

//         res
//     }
// }

// impl<const N: usize> SubAssign for Ad<N> {
//     fn sub_assign(&mut self, rhs: Self) {
//         *self = self.clone() - rhs;
//     }
// }

// impl<const N: usize> Mul for Ad<N> {
//     type Output = Self;

//     fn mul(self, rhs: Self) -> Self::Output {
//         let mut res = Self::_zeroed();
//         res.value = self.value * rhs.value;
//         res.grad = self.grad * rhs.value + self.value * rhs.grad;
//         res.hess = rhs.value * self.hess
//             + self.value * rhs.hess
//             + self.grad * rhs.grad.transpose()
//             + rhs.grad * self.grad.transpose();

//         res
//     }
// }

// impl<const N: usize> MulAssign for Ad<N> {
//     fn mul_assign(&mut self, rhs: Self) {
//         *self = self.clone() * rhs;
//     }
// }

// impl<const N: usize> Div for Ad<N> {
//     type Output = Self;

//     fn div(self, rhs: Self) -> Self {
//         if rhs.value.abs() == 0.0 {
//             // We don't want to mute this behavior or get NaN as this is fucking undebuggable.
//             panic!("Division By Zero!");
//         }

//         let mut res = Self::_zeroed();
//         res.value = self.value / rhs.value;
//         res.grad = (rhs.value * self.grad - self.value * rhs.grad) / (rhs.value * rhs.value);
//         res.hess = (self.hess
//             - res.grad * rhs.grad.transpose()
//             - rhs.grad * res.grad.transpose()
//             - res.value * rhs.hess)
//             / rhs.value;

//         res
//     }
// }

// impl<const N: usize> DivAssign for Ad<N> {
//     fn div_assign(&mut self, rhs: Self) {
//         *self = self.clone() / rhs;
//     }
// }

// impl<const N: usize> Rem for Ad<N> {
//     type Output = Self;

//     fn rem(self, rhs: Self) -> Self {
//         unimplemented!()
//     }
// }

// impl<const N: usize> RemAssign for Ad<N> {
//     fn rem_assign(&mut self, rhs: Self) {
//         unimplemented!()
//     }
// }

*/
