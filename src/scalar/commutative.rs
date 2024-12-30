use core::panic;

use crate::Ad;

pub trait Commutative<const N: usize> {
    type Ad;
    fn div_ad(&self, denom: &Self::Ad) -> Self::Ad;
    fn pow_ad(&self, exponent: &Self::Ad) -> Self::Ad;
}

impl<const N: usize> Commutative<N> for f64 {
    type Ad = Ad<N>;

    fn div_ad(&self, denom: &Self::Ad) -> Self::Ad {
        if denom.value.abs() == 0.0 {
            // We don't want to mute this behavior or get NaN as this is fucking undebuggable.
            panic!("Division By Zero!");
        }

        let mut res = Self::Ad::_zeroed();
        res.value = self / denom.value;
        res.grad = (-self / (denom.value * denom.value)) * denom.grad;
        res.hess = (-res.grad * denom.grad.transpose()
            - denom.grad * res.grad.transpose()
            - res.value * denom.hess)
            / denom.value;

        res
    }

    /// Not tested.
    fn pow_ad(&self, exponent: &Self::Ad) -> Self::Ad {
        if *self < 0.0 {
            panic!("pow_ad out of domain!");
        }

        let mut res = Self::Ad::_zeroed();

        let powered = self.powf(exponent.value);
        let logged = self.ln();

        res.value = powered;
        res.grad = powered * logged * exponent.grad;
        res.hess = powered * logged * logged * exponent.hess;

        res
    }
}
