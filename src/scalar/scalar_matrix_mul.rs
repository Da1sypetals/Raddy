/*

This code is generated by meta/scalar_matrix_mul.py at 12:19:50 @ 2025.01.08
Do not modify it directly.

*/
use crate::Ad;
use na::SMatrix;
use std::ops::Mul;

impl<const N: usize, const R: usize, const C: usize> Mul<SMatrix<Ad<N>, R, C>> for Ad<N> {
    type Output = SMatrix<Ad<N>, R, C>;

    fn mul(self, rhs: SMatrix<Ad<N>, R, C>) -> Self::Output {
        rhs * self
    }
}

impl<const N: usize, const R: usize, const C: usize> Mul<SMatrix<Ad<N>, R, C>> for &Ad<N> {
    type Output = SMatrix<Ad<N>, R, C>;

    fn mul(self, rhs: SMatrix<Ad<N>, R, C>) -> Self::Output {
        rhs * self.clone()
    }
}

impl<const N: usize, const R: usize, const C: usize> Mul<&SMatrix<Ad<N>, R, C>> for Ad<N> {
    type Output = SMatrix<Ad<N>, R, C>;

    fn mul(self, rhs: &SMatrix<Ad<N>, R, C>) -> Self::Output {
        rhs * self
    }
}

impl<const N: usize, const R: usize, const C: usize> Mul<&SMatrix<Ad<N>, R, C>> for &Ad<N> {
    type Output = SMatrix<Ad<N>, R, C>;

    fn mul(self, rhs: &SMatrix<Ad<N>, R, C>) -> Self::Output {
        rhs * self.clone()
    }
}
