use crate::{types::admat, Ad};
use itertools::Itertools;
use na::{SMatrix, SVector};

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
pub fn scalar<const N: usize>(value: f64) -> Ad<N> {
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
pub fn vector_from_slice<const N: usize, const L: usize>(values: &[f64]) -> SVector<Ad<N>, L> {
    Ad::inactive_from_slice(values)
}

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
pub fn vectpr<const N: usize, const L: usize>(values: &[f64]) -> SVector<Ad<N>, L> {
    Ad::inactive_from_slice(values)
}

/// Creates a matrix of inactive Ad values from a row-major slice
///
/// # Arguments
/// * `values` - Slice of f64 values in row-major order to convert to inactive Ad values
///
/// # Type Parameters
/// * `N` - The input dimension (for derivatives)
/// * `R` - The number of rows
/// * `C` - The number of columns
///
/// # Panics
/// Panics if the length of values does not equal R * C
///
/// # Returns
/// An SMatrix of `Ad<N>` values where each element is inactive
pub fn matrix_from_row_slice<const N: usize, const R: usize, const C: usize>(
    values: &[f64],
) -> SMatrix<Ad<N>, R, C> {
    assert_eq!(
        values.len(),
        R * C,
        "Slice mismatch matrix dimensions: found {}, expected : {} ({}x{})",
        values.len(),
        R * C,
        R,
        C
    );
    let mut res: admat<N, R, C> = SMatrix::zeros();

    for (r, c) in (0..R).cartesian_product(0..C) {
        res[(r, c)].value = values[r * C + c];
    }

    res
}

/// Creates a matrix of inactive Ad values from a column-major slice
///
/// # Arguments
/// * `values` - Slice of f64 values in column-major order to convert to inactive Ad values
///
/// # Type Parameters
/// * `N` - The input dimension (for derivatives)
/// * `R` - The number of rows
/// * `C` - The number of columns
///
/// # Panics
/// Panics if the length of values does not equal R * C
///
/// # Returns
/// An SMatrix of `Ad<N>` values where each element is inactive
pub fn matrix_from_column_slice<const N: usize, const R: usize, const C: usize>(
    values: &[f64],
) -> SMatrix<Ad<N>, R, C> {
    assert_eq!(
        values.len(),
        R * C,
        "Slice mismatch matrix dimensions: found {}, expected: {} ({}x{})",
        values.len(),
        R * C,
        R,
        C
    );
    let mut res: admat<N, R, C> = SMatrix::zeros();

    for (r, c) in (0..R).cartesian_product(0..C) {
        res[(r, c)].value = values[c * R + r];
    }

    res
}

/// Converts a matrix of f64 values to a matrix of inactive Ad values
///
/// # Arguments
/// * `matrix` - The matrix of f64 values to convert
///
/// # Type Parameters
/// * `N` - The input dimension (for derivatives)
/// * `R` - The number of rows
/// * `C` - The number of columns
///
/// # Returns
/// An SMatrix of `Ad<N>` values where each element is inactive
pub fn valmat<const N: usize, const R: usize, const C: usize>(
    matrix: SMatrix<f64, R, C>,
) -> SMatrix<Ad<N>, R, C> {
    let mut res: admat<N, R, C> = SMatrix::zeros();

    for (r, c) in (0..R).cartesian_product(0..C) {
        res[(r, c)].value = matrix[(r, c)];
    }

    res
}
