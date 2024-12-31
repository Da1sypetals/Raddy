from datetime import datetime


def neg(ref):
    t = "&" if ref else ""
    code = f"""
impl<const N: usize> Neg for {t}Ad<N> {{
    type Output = Ad<N>;

    fn neg(self) -> Ad<N> {{
        let mut res = Ad::<N>::_zeroed();
        res.value = -self.value;
        res.grad = -self.grad;
        res.hess = -self.hess;

        res
    }}
}}
"""
    return code


def add(l_ref, r_ref):
    left = "&" if l_ref else ""
    right = "&" if r_ref else ""

    code = f"""
// {left}T + {right}T
impl<const N: usize> Add<{right}Ad<N>> for {left}Ad<N> {{
    type Output = Ad<N>;

    fn add(self, rhs: {right}Ad<N>) -> Self::Output {{
        let mut res = Ad::<N>::_zeroed();
        res.value = self.value + rhs.value;
        res.grad = self.grad + rhs.grad;
        res.hess = self.hess + rhs.hess;

        res
    }}
}}

"""

    return code


def sub(l_ref, r_ref):
    left = "&" if l_ref else ""
    right = "&" if r_ref else ""

    code = f"""
// {left}T - {right}T
impl<const N: usize> Sub<{right}Ad<N>> for {left}Ad<N> {{
    type Output = Ad<N>;

    fn sub(self, rhs: {right}Ad<N>) -> Self::Output {{
        let mut res = Ad::<N>::_zeroed();
        res.value = self.value - rhs.value;
        res.grad = self.grad - rhs.grad;
        res.hess = self.hess - rhs.hess;

        res
    }}
}}

"""

    return code


def mul(l_ref, r_ref):
    left = "&" if l_ref else ""
    right = "&" if r_ref else ""

    code = f"""
// {left}T * {right}T
impl<const N: usize> Mul<{right}Ad<N>> for {left}Ad<N> {{
    type Output = Ad<N>;

    fn mul(self, rhs: {right}Ad<N>) -> Self::Output {{
        let mut res = Ad::<N>::_zeroed();

        res.value = self.value * rhs.value;
        res.grad = self.grad * rhs.value + self.value * rhs.grad;
        res.hess = rhs.value * self.hess
            + self.value * rhs.hess
            + self.grad * rhs.grad.transpose()
            + rhs.grad * self.grad.transpose();

        res
    }}
}}

"""

    return code


def div(l_ref, r_ref):
    left = "&" if l_ref else ""
    right = "&" if r_ref else ""

    code = f"""
// {left}T / {right}T
impl<const N: usize> Div<{right}Ad<N>> for {left}Ad<N> {{
    type Output = Ad<N>;

    fn div(self, rhs: {right}Ad<N>) -> Self::Output {{
        if rhs.value.abs() == 0.0 {{
            // We don't want to mute this behavior or get NaN as this is fucking undebuggable.
            panic!("Division By Zero!");
        }}

        let mut res = Ad::<N>::_zeroed();
        res.value = self.value / rhs.value;
        res.grad = (rhs.value * self.grad - self.value * rhs.grad) / (rhs.value * rhs.value);
        res.hess = (self.hess
            - res.grad * rhs.grad.transpose()
            - rhs.grad * res.grad.transpose()
            - res.value * rhs.hess)
            / rhs.value;

        res
    }}
}}

"""

    return code


def rem(l_ref, r_ref):
    left = "&" if l_ref else ""
    right = "&" if r_ref else ""

    code = f"""
// {left}T % {right}T
impl<const N: usize> Rem<{right}Ad<N>> for {left}Ad<N> {{
    type Output = Ad<N>;

    fn rem(self, rhs: {right}Ad<N>) -> Self::Output {{
        unimplemented!();
    }}
}}

"""

    return code


def op_assign(cased_op_name, operator, r_ref):
    right = "&" if r_ref else ""

    code = f"""
// T {operator}= {right}T
impl<const N: usize> {cased_op_name}Assign<{right}Ad<N>> for Ad<N> {{
    fn {cased_op_name.lower()}_assign(&mut self, rhs: {right}Ad<N>) {{
        *self = self.clone() {operator} rhs;
    }}
}}


"""

    return code


def rem_assign_unimpl(r_ref):
    right = "&" if r_ref else ""

    code = f"""
// T %= {right}T
impl<const N: usize> RemAssign<{right}Ad<N>> for Ad<N> {{
    fn rem_assign(&mut self, rhs: {right}Ad<N>) {{
        unimplemented!();
    }}
}}


"""

    return code


if __name__ == "__main__":
    now = datetime.now()
    formatted_date = now.strftime("%H:%M:%S @ %Y.%m.%d")
    res = f"""/*

This code is generated by meta/operators.py at {formatted_date}
Do not modify it directly.

*/

#![allow(unused)]

use crate::Ad;
use std::ops::{{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign}};
"""

    res += neg(True)
    res += neg(False)

    comb = [[True, True], [True, False], [False, True], [False, False]]

    for ll, rr in comb:
        res += add(ll, rr)

    for ll, rr in comb:
        res += sub(ll, rr)

    for ll, rr in comb:
        res += mul(ll, rr)

    for ll, rr in comb:
        res += div(ll, rr)

    for ll, rr in comb:
        res += rem(ll, rr)

    ops = [
        ["Add", "+"],
        ["Sub", "-"],
        ["Mul", "*"],
        ["Div", "/"],
    ]

    for name, opr in ops:
        res += op_assign(name, opr, True)
        res += op_assign(name, opr, False)

    res += rem_assign_unimpl(True)
    res += rem_assign_unimpl(False)

    with open("src/scalar/operator_traits_impl.rs", "w") as sf:
        sf.write(res)
