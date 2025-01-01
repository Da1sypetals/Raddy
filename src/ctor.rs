use crate::Ad;
use na::SVector;

#[inline]
pub fn ad(value: f64) -> Ad<1> {
    Ad::active_scalar(value)
}

#[inline]
pub fn val(value: f64) -> Ad<1> {
    Ad::inactive_scalar(value)
}

#[inline]
pub fn vec<const L: usize>(values: &[f64]) -> SVector<Ad<L>, L> {
    Ad::active_from_slice(values)
}

#[inline]
pub fn valvec<const L: usize>(values: &[f64]) -> SVector<Ad<L>, L> {
    Ad::inactive_from_slice(values)
}
