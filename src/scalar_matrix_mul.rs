/*

This code is generated by meta/scalar_matrix_mul.py at 12:29:15 @ 2024.12.29
Do not modify it directly.

*/
use crate::Scalar;
use nalgebra::SMatrix;
use std::ops::Mul;

impl<const N: usize, const R: usize, const C: usize> Mul<SMatrix<Scalar<N>, R, C>> for Scalar<N> {
    type Output = SMatrix<Scalar<N>, R, C>;

    fn mul(self, rhs: SMatrix<Scalar<N>, R, C>) -> Self::Output {
        let mut res = rhs.clone();
        res *= self.clone();
        return res;
    }
}

impl<const N: usize, const R: usize, const C: usize> Mul<SMatrix<Scalar<N>, R, C>> for &Scalar<N> {
    type Output = SMatrix<Scalar<N>, R, C>;

    fn mul(self, rhs: SMatrix<Scalar<N>, R, C>) -> Self::Output {
        let mut res = rhs.clone();
        res *= self.clone();
        return res;
    }
}

impl<const N: usize, const R: usize, const C: usize> Mul<&SMatrix<Scalar<N>, R, C>> for Scalar<N> {
    type Output = SMatrix<Scalar<N>, R, C>;

    fn mul(self, rhs: &SMatrix<Scalar<N>, R, C>) -> Self::Output {
        let mut res = rhs.clone();
        res *= self.clone();
        return res;
    }
}

impl<const N: usize, const R: usize, const C: usize> Mul<&SMatrix<Scalar<N>, R, C>> for &Scalar<N> {
    type Output = SMatrix<Scalar<N>, R, C>;

    fn mul(self, rhs: &SMatrix<Scalar<N>, R, C>) -> Self::Output {
        let mut res = rhs.clone();
        res *= self.clone();
        return res;
    }
}
