use crate::Ad;
use na::SVector;

#[inline]
/// Creates an active scalar Ad value with single input dimension
///
/// # Arguments
/// * `value` - The scalar value to wrap in an Ad type
///
/// # Returns
/// An Ad<1> instance representing an active scalar value with single input dimension
pub fn scalar(value: f64) -> Ad<1> {
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
pub fn vector_from_slice<const L: usize>(values: &[f64]) -> SVector<Ad<L>, L> {
    Ad::active_from_slice(values)
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
pub fn vector<const L: usize>(vector: &SVector<f64, L>) -> SVector<Ad<L>, L> {
    Ad::active_vector(vector)
}
