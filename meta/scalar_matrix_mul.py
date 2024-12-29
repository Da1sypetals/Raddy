from datetime import datetime


def scalar_matrix_mul_template(scalar_ref, matrix_ref):
    scalar = "&" if scalar_ref else ""
    matrix = "&" if matrix_ref else ""

    return f"""
impl<const N: usize, const R: usize, const C: usize> Mul<{matrix}SMatrix<Ad<N>, R, C>> for {scalar}Ad<N> {{
    type Output = SMatrix<Ad<N>, R, C>;

    fn mul(self, rhs: {matrix}SMatrix<Ad<N>, R, C>) -> Self::Output {{
        let mut res = rhs.clone();
        res *= self.clone();
        return res;
    }}
}}
"""


if __name__ == "__main__":
    now = datetime.now()
    formatted_date = now.strftime("%H:%M:%S @ %Y.%m.%d")
    res = f"""/*

This code is generated by meta/scalar_matrix_mul.py at {formatted_date}
Do not modify it directly.

*/
use crate::Ad;
use nalgebra::SMatrix;
use std::ops::Mul;
"""
    res += scalar_matrix_mul_template(scalar_ref=False, matrix_ref=False)
    res += scalar_matrix_mul_template(scalar_ref=True, matrix_ref=False)
    res += scalar_matrix_mul_template(scalar_ref=False, matrix_ref=True)
    res += scalar_matrix_mul_template(scalar_ref=True, matrix_ref=True)
    with open("src/scalar_matrix_mul.rs", "w") as sf:
        sf.write(res)
