use crate::Ad;
use nalgebra::SMatrix;

/// Self is matrix type; Scalar is scalar type.
pub trait AdMatrixOps {
    type Scalar;

    // Norms
    fn l1_norm(&self) -> Self::Scalar;
    fn l2_norm(&self) -> Self::Scalar;
    fn l2_norm_squared(&self) -> Self::Scalar;
    fn lk_norm(&self, k: u32) -> Self::Scalar;
    fn linf_norm(&self) -> Self::Scalar;

    // Operator extensions
    fn scale(&self, factor: f64) -> Self;
}

impl<const N: usize, const R: usize, const C: usize> AdMatrixOps for SMatrix<Ad<N>, R, C> {
    type Scalar = Ad<N>;

    /// Sum of absolute of all components
    fn l1_norm(&self) -> Self::Scalar {
        let mut res = Ad::_zeroed();
        for r in 0..R {
            for c in 0..C {
                res += self[(r, c)].abs().clone();
            }
        }
        res
    }

    /// Pythagoras theorem
    fn l2_norm(&self) -> Self::Scalar {
        let mut res = Ad::_zeroed();
        for r in 0..R {
            for c in 0..C {
                res += self[(r, c)].square();
            }
        }
        res.sqrt()
    }

    fn l2_norm_squared(&self) -> Self::Scalar {
        let mut res = Ad::_zeroed();
        for r in 0..R {
            for c in 0..C {
                res += self[(r, c)].square();
            }
        }
        res
    }

    fn lk_norm(&self, k: u32) -> Self::Scalar {
        let mut res = Ad::_zeroed();
        for r in 0..R {
            for c in 0..C {
                res += self[(r, c)].powi(k as i32);
            }
        }
        res.powf(1.0 / (k as f64))
    }

    fn linf_norm(&self) -> Self::Scalar {
        let mut res = self[0].clone();
        for r in 0..R {
            for c in 0..C {
                res = res.max(&self[(r, c)].abs().clone());
            }
        }
        res
    }

    fn scale(&self, factor: f64) -> Self {
        Ad::inactive_scalar(factor) * self
    }
}
