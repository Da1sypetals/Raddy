use crate::Scalar;

pub trait Commutative<const N: usize> {
    type Var;
    fn div_var(&self, var: &Self::Var) -> Self::Var;
}

impl<const N: usize> Commutative<N> for f64 {
    type Var = Scalar<N>;
    fn div_var(&self, var: &Self::Var) -> Self::Var {
        if var.value.abs() == 0.0 {
            // We don't want to mute this behavior or get NaN as this is fucking undebuggable.
            panic!("Division By Zero!");
        }

        let mut res = Self::Var::_zeroed();
        res.value = self / var.value;
        res.grad = (-self / (var.value * var.value)) * var.grad;
        res.hess = (-res.grad * var.grad.transpose()
            - var.grad * res.grad.transpose()
            - res.value * var.hess)
            / var.value;

        res
    }
}
