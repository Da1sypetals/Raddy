use crate::Ad;
use na::SMatrix;

pub trait AdMatrixOps {
    fn scale(&self, factor: f64) -> Self;
}

impl<const N: usize, const R: usize, const C: usize> AdMatrixOps for SMatrix<Ad<N>, R, C> {
    fn scale(&self, factor: f64) -> Self {
        Ad::inactive_scalar(factor) * self
    }
}
