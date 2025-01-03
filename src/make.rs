use crate::Ad;
use na::SVector;

/// Utility functions for creating automatic differentiation (Ad) values and vectors

#[inline]
/// Creates an active scalar Ad value with single input dimension
///
/// # Arguments
/// * `value` - The scalar value to wrap in an Ad type
///
/// # Returns
/// An Ad<1> instance representing an active scalar value with single input dimension
pub fn ad(value: f64) -> Ad<1> {
    Ad::active_scalar(value)
}

#[inline]
/// Creates a vector of active Ad values with specified input and vector dimensions
///
/// # Arguments
/// * `values` - Slice of f64 values to convert to active Ad values
///
/// # Type Parameters
/// * `L` - Both the input dimension (for gradients) and vector length
///
/// # Returns
/// An SVector of `Ad<L>` values where each element is active
pub fn vec<const L: usize>(values: &[f64]) -> SVector<Ad<L>, L> {
    Ad::active_from_slice(values)
}

#[inline]
/// Creates an inactive scalar Ad value with specified inputd dimensions
///
/// # Arguments
/// * `value` - The scalar value to wrap in an Ad type
///
/// # Type Parameters
/// * `N` - The input dimension (for derivatives)
///
/// # Returns
/// An `Ad<N>` instance representing an inactive scalar value
pub fn val<const N: usize>(value: f64) -> Ad<N> {
    Ad::inactive_scalar(value)
}

#[inline]
/// Creates a vector of inactive Ad values with separate input and vector dimensions
///
/// # Arguments
/// * `values` - Slice of f64 values to convert to inactive Ad values
///
/// # Type Parameters
/// * `N` - The input dimension (for derivatives)
/// * `L` - The length of the vector
///
/// # Returns
/// An SVector of `Ad<N>` values where each element is inactive
pub fn valvec<const N: usize, const L: usize>(values: &[f64]) -> SVector<Ad<N>, L> {
    Ad::inactive_from_slice(values)
}
