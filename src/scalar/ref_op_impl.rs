use std::ops::{Add, Mul};

use crate::Ad;

use super::commutative::Commutative;

// ################################### Unary Operators ###################################

impl<const N: usize> Ad<N> {
    pub fn neg(&self) -> Self {
        let mut res = Self::_zeroed();
        res.value = -self.value;
        res.grad = -self.grad;
        res.hess = -self.hess;

        res
    }

    pub fn sqrt(&self) -> Self {
        if self.value < -0.0 {
            // We don't want to mute this behavior or get NaN as this is fucking undebuggable.
            panic!("Sqrt on negative value!");
        }
        let f = self.value.sqrt();

        Self::chain(f, 0.5 / f, -0.25 / (f * self.value), self)
    }

    pub fn square(&self) -> Self {
        let mut res = Self::_zeroed();
        res.value = self.value * self.value;
        res.grad = 2.0 * self.value * self.grad;
        res.hess = 2.0 * (self.value * self.hess + self.grad * self.grad.transpose());

        res
    }

    pub fn powi(&self, exponent: i32) -> Self {
        if self.value.abs() == 0.0 && exponent == 0 {
            // We don't want to mute this behavior or get NaN as this is fucking undebuggable.
            panic!("0.pow(0) is undefined!");
        }

        let f2 = self.value.powi(exponent - 2);
        let f1 = f2 * self.value;
        let f = f1 * self.value;

        // exponent in float
        let ef = exponent as f64;

        Self::chain(f, ef * f1, ef * (ef - 1.0) * f2, self)
    }

    pub fn powf(&self, exponent: f64) -> Self {
        if self.value.abs() == 0.0 && exponent.abs() == 0.0 {
            // We don't want to mute this behavior or get NaN as this is fucking undebuggable.
            panic!("0.pow(0) is undefined!");
        }

        let f2 = self.value.powf(exponent - 2.0);
        let f1 = f2 * self.value;
        let f = f1 * self.value;

        // exponent in float

        Self::chain(f, exponent * f1, exponent * (exponent - 1.0) * f2, self)
    }

    pub fn abs(&self) -> Self {
        let mut res = Self::_zeroed();
        res.value = self.value.abs();
        let sign = if self.value >= 0.0 { 1.0 } else { -1.0 };
        res.grad = sign * self.grad;
        res.hess = sign * self.hess;

        res
    }

    pub fn exp(&self) -> Self {
        let exp_val = self.value.exp();

        Self::chain(exp_val, exp_val, exp_val, self)
    }

    pub fn ln(&self) -> Self {
        if self.value <= 0.0 {
            panic!("Ln on non-positive value!");
        }
        let inv = 1.0 / self.value;

        Self::chain(self.value.ln(), inv, -inv * inv, self)
    }

    pub fn log(&self, base: f64) -> Self {
        if self.value <= 0.0 {
            panic!("Log2 on non-positive value!");
        }
        if base <= 0.0 {
            panic!("Base must be positive!");
        }

        let inv = 1.0 / self.value / base.ln();

        Self::chain(self.value.log(base), inv, -inv / self.value, self)
    }

    pub fn log2(&self) -> Self {
        if self.value <= 0.0 {
            panic!("Log2 on non-positive value!");
        }
        let inv = 1.0 / self.value / std::f64::consts::LN_2;

        Self::chain(self.value.log2(), inv, -inv / self.value, self)
    }

    pub fn log10(&self) -> Self {
        if self.value <= 0.0 {
            panic!("Log10 on non-positive value!");
        }
        let inv = 1.0 / self.value / std::f64::consts::LN_10;

        Self::chain(self.value.log10(), inv, -inv / self.value, self)
    }

    pub fn sin(&self) -> Self {
        let sin_val = self.value.sin();
        let cos_val = self.value.cos();

        Self::chain(sin_val, cos_val, -sin_val, self)
    }

    pub fn cos(&self) -> Self {
        let cos_val = self.value.cos();
        let sin_val = self.value.sin();

        Self::chain(cos_val, -sin_val, -cos_val, self)
    }

    pub fn tan(&self) -> Self {
        let cos_val = self.value.cos();
        let cos_sq = cos_val * cos_val;

        Self::chain(
            self.value.tan(),
            1.0 / cos_sq,
            2.0 * self.value.sin() / (cos_sq * cos_val),
            self,
        )
    }

    pub fn asin(&self) -> Self {
        if self.value < -1.0 || self.value > 1.0 {
            panic!("Asin out of domain!");
        }
        let s = 1.0 - self.value * self.value;
        let s_sqrt = s.sqrt();

        Self::chain(
            self.value.asin(),
            1.0 / s_sqrt,
            self.value / (s * s_sqrt),
            self,
        )
    }

    pub fn acos(&self) -> Self {
        if self.value < -1.0 || self.value > 1.0 {
            panic!("Acos out of domain!");
        }
        let s = 1.0 - self.value * self.value;
        let s_sqrt = s.sqrt();

        Self::chain(
            self.value.acos(),
            -1.0 / s_sqrt,
            -self.value / (s * s_sqrt),
            self,
        )
    }

    #[deprecated = "Please use atan2 instead."]
    pub fn atan(&self) -> Self {
        let s = self.value * self.value + 1.0;

        Self::chain(
            self.value.atan(),
            1.0 / s,
            -2.0 * self.value / (s * s),
            self,
        )
    }

    pub fn sinh(&self) -> Self {
        let sinh_val = self.value.sinh();
        let cosh_val = self.value.cosh();

        Self::chain(sinh_val, cosh_val, sinh_val, self)
    }

    pub fn cosh(&self) -> Self {
        let sinh_val = self.value.sinh();
        let cosh_val = self.value.cosh();

        Self::chain(cosh_val, sinh_val, cosh_val, self)
    }

    pub fn tanh(&self) -> Self {
        let cosh_val = self.value.cosh();
        let cosh_sq = cosh_val * cosh_val;

        Self::chain(
            self.value.tanh(),
            1.0 / cosh_sq,
            -2.0 * self.value.sinh() / (cosh_sq * cosh_val),
            self,
        )
    }

    pub fn asinh(&self) -> Self {
        let s = self.value * self.value + 1.0;
        let s_sqrt = s.sqrt();

        Self::chain(
            self.value.asinh(),
            1.0 / s_sqrt,
            -self.value / (s * s_sqrt),
            self,
        )
    }

    pub fn acosh(&self) -> Self {
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
            self,
        )
    }

    pub fn atanh(&self) -> Self {
        if self.value <= -1.0 || self.value >= 1.0 {
            panic!("Atanh out of domain!");
        }
        let s = 1.0 - self.value * self.value;

        Self::chain(
            self.value.atanh(),
            1.0 / s,
            2.0 * self.value / (s * s),
            self,
        )
    }
}

// ################################### Binary Operators ###################################

impl<const N: usize> Ad<N> {
    pub fn add_value(&self, other: f64) -> Self {
        let mut res = Self::_zeroed();
        res.value = self.value + other;
        res
    }

    pub fn sub_value(&self, other: f64) -> Self {
        let mut res = Self::_zeroed();
        res.value = self.value - other;
        res
    }

    pub fn mul_value(&self, other: f64) -> Self {
        let mut res = Self::_zeroed();
        res.value = self.value * other;
        res.grad = self.grad * other;
        res.hess = self.hess * other;

        res
    }

    pub fn recip(&self) -> Self {
        1_f64.div_ad(self)
    }

    pub fn div_value(&self, other: f64) -> Self {
        if other.abs() == 0.0 {
            // We don't want to mute this behavior or get NaN as this is fucking undebuggable.
            panic!("Division By Zero!");
        }

        let mut res = Self::_zeroed();
        res.value = self.value / other;
        res.grad = self.grad / other;
        res.hess = self.hess / other;

        res
    }

    /// ## self is y
    pub fn atan2(&self, x: &Self) -> Self {
        let mut res = Self::_zeroed();

        // Compute scalar value of atan2
        res.value = self.value.atan2(x.value);

        // Gradient computation
        let u = x.value * &self.grad - self.value * &x.grad;
        let v = x.value * x.value + self.value * self.value;
        res.grad = &u / v;

        // Hessian computation (if enabled)
        let du = x.value * &self.hess - self.value * &x.hess + &self.grad * x.grad.transpose()
            - &x.grad * self.grad.transpose();
        let dv = 2.0 * (x.value * &x.grad + self.value * &self.grad);
        res.hess = (&du - &res.grad * dv.transpose()) / v;

        res
    }

    pub fn min(&self, other: &Self) -> Self {
        if self < other {
            self.clone()
        } else {
            other.clone()
        }
    }

    pub fn max(&self, other: &Self) -> Self {
        if self > other {
            self.clone()
        } else {
            other.clone()
        }
    }

    pub fn clamp(&self, low: &Self, high: &Self) -> Self {
        self.max(low).min(high)
    }

    // Computes hypot(self, b) = sqrt(self^2 + b^2) with gradients and Hessians if enabled.
    pub fn hypot(&self, other: &Self) -> Self {
        (self.mul(self).add(&other.mul(other))).sqrt()
    }
}
