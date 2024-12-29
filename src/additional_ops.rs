use crate::{
    det::{det_4_4_lu, det_5_5, det_6_6_lu},
    Ad,
};
use nalgebra::SMatrix;
use num_traits::One;

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
    fn determinant(&self) -> Self::Scalar;
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

    fn determinant(&self) -> Self::Scalar {
        let dim = self.shape().0;

        unsafe {
            match dim {
                0 => Ad::<N>::one(),
                1 => self.get_unchecked((0, 0)).clone(),
                2 => {
                    let m11 = self.get_unchecked((0, 0)).clone();
                    let m12 = self.get_unchecked((0, 1)).clone();
                    let m21 = self.get_unchecked((1, 0)).clone();
                    let m22 = self.get_unchecked((1, 1)).clone();

                    m11 * m22 - m21 * m12
                }
                3 => {
                    let m11 = self.get_unchecked((0, 0)).clone();
                    let m12 = self.get_unchecked((0, 1)).clone();
                    let m13 = self.get_unchecked((0, 2)).clone();

                    let m21 = self.get_unchecked((1, 0)).clone();
                    let m22 = self.get_unchecked((1, 1)).clone();
                    let m23 = self.get_unchecked((1, 2)).clone();

                    let m31 = self.get_unchecked((2, 0)).clone();
                    let m32 = self.get_unchecked((2, 1)).clone();
                    let m33 = self.get_unchecked((2, 2)).clone();

                    let minor_m12_m23 = m22.clone() * m33.clone() - m32.clone() * m23.clone();
                    let minor_m11_m23 = m21.clone() * m33 - m31.clone() * m23;
                    let minor_m11_m22 = m21 * m32 - m31 * m22;

                    m11 * minor_m12_m23 - m12 * minor_m11_m23 + m13 * minor_m11_m22
                }
                4 => {
                    let a0 = self.get_unchecked((0, 0)).clone();
                    let a1 = self.get_unchecked((0, 1)).clone();
                    let a2 = self.get_unchecked((0, 2)).clone();
                    let a3 = self.get_unchecked((0, 3)).clone();
                    let a4 = self.get_unchecked((1, 0)).clone();
                    let a5 = self.get_unchecked((1, 1)).clone();
                    let a6 = self.get_unchecked((1, 2)).clone();
                    let a7 = self.get_unchecked((1, 3)).clone();
                    let a8 = self.get_unchecked((2, 0)).clone();
                    let a9 = self.get_unchecked((2, 1)).clone();
                    let a10 = self.get_unchecked((2, 2)).clone();
                    let a11 = self.get_unchecked((2, 3)).clone();
                    let a12 = self.get_unchecked((3, 0)).clone();
                    let a13 = self.get_unchecked((3, 1)).clone();
                    let a14 = self.get_unchecked((3, 2)).clone();
                    let a15 = self.get_unchecked((3, 3)).clone();

                    det_4_4_lu(
                        a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15,
                    )
                }
                5 => {
                    let a0 = self.get_unchecked((0, 0)).clone();
                    let a1 = self.get_unchecked((0, 1)).clone();
                    let a2 = self.get_unchecked((0, 2)).clone();
                    let a3 = self.get_unchecked((0, 3)).clone();
                    let a4 = self.get_unchecked((0, 4)).clone();
                    let a5 = self.get_unchecked((1, 0)).clone();
                    let a6 = self.get_unchecked((1, 1)).clone();
                    let a7 = self.get_unchecked((1, 2)).clone();
                    let a8 = self.get_unchecked((1, 3)).clone();
                    let a9 = self.get_unchecked((1, 4)).clone();
                    let a10 = self.get_unchecked((2, 0)).clone();
                    let a11 = self.get_unchecked((2, 1)).clone();
                    let a12 = self.get_unchecked((2, 2)).clone();
                    let a13 = self.get_unchecked((2, 3)).clone();
                    let a14 = self.get_unchecked((2, 4)).clone();
                    let a15 = self.get_unchecked((3, 0)).clone();
                    let a16 = self.get_unchecked((3, 1)).clone();
                    let a17 = self.get_unchecked((3, 2)).clone();
                    let a18 = self.get_unchecked((3, 3)).clone();
                    let a19 = self.get_unchecked((3, 4)).clone();
                    let a20 = self.get_unchecked((4, 0)).clone();
                    let a21 = self.get_unchecked((4, 1)).clone();
                    let a22 = self.get_unchecked((4, 2)).clone();
                    let a23 = self.get_unchecked((4, 3)).clone();
                    let a24 = self.get_unchecked((4, 4)).clone();

                    det_5_5(
                        a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16,
                        a17, a18, a19, a20, a21, a22, a23, a24,
                    )
                }
                6 => {
                    let a0 = self.get_unchecked((0, 0)).clone();
                    let a1 = self.get_unchecked((0, 1)).clone();
                    let a2 = self.get_unchecked((0, 2)).clone();
                    let a3 = self.get_unchecked((0, 3)).clone();
                    let a4 = self.get_unchecked((0, 4)).clone();
                    let a5 = self.get_unchecked((0, 5)).clone();
                    let a6 = self.get_unchecked((1, 0)).clone();
                    let a7 = self.get_unchecked((1, 1)).clone();
                    let a8 = self.get_unchecked((1, 2)).clone();
                    let a9 = self.get_unchecked((1, 3)).clone();
                    let a10 = self.get_unchecked((1, 4)).clone();
                    let a11 = self.get_unchecked((1, 5)).clone();
                    let a12 = self.get_unchecked((2, 0)).clone();
                    let a13 = self.get_unchecked((2, 1)).clone();
                    let a14 = self.get_unchecked((2, 2)).clone();
                    let a15 = self.get_unchecked((2, 3)).clone();
                    let a16 = self.get_unchecked((2, 4)).clone();
                    let a17 = self.get_unchecked((2, 5)).clone();
                    let a18 = self.get_unchecked((3, 0)).clone();
                    let a19 = self.get_unchecked((3, 1)).clone();
                    let a20 = self.get_unchecked((3, 2)).clone();
                    let a21 = self.get_unchecked((3, 3)).clone();
                    let a22 = self.get_unchecked((3, 4)).clone();
                    let a23 = self.get_unchecked((3, 5)).clone();
                    let a24 = self.get_unchecked((4, 0)).clone();
                    let a25 = self.get_unchecked((4, 1)).clone();
                    let a26 = self.get_unchecked((4, 2)).clone();
                    let a27 = self.get_unchecked((4, 3)).clone();
                    let a28 = self.get_unchecked((4, 4)).clone();
                    let a29 = self.get_unchecked((4, 5)).clone();
                    let a30 = self.get_unchecked((5, 0)).clone();
                    let a31 = self.get_unchecked((5, 1)).clone();
                    let a32 = self.get_unchecked((5, 2)).clone();
                    let a33 = self.get_unchecked((5, 3)).clone();
                    let a34 = self.get_unchecked((5, 4)).clone();
                    let a35 = self.get_unchecked((5, 5)).clone();

                    det_6_6_lu(
                        a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16,
                        a17, a18, a19, a20, a21, a22, a23, a24, a25, a26, a27, a28, a29, a30, a31,
                        a32, a33, a34, a35,
                    )
                }
                _ => {
                    unimplemented!(
                        "The cost of forward mode AD of determinant with dim > 6 is too huge"
                    );
                }
            }
        }
    }
}
