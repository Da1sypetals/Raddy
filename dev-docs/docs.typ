// ---------- Configurations ----------
#set page(
  paper: "us-letter",
  numbering: "1",
)
#set par(justify: true)

#set text(
  font: (
    "Libertinus Serif",
    "STZhongsong"
  ),
  lang: "en", region: "us",
  size: 12pt,
)

#set text(top-edge: 0.7em, bottom-edge: -0.3em)
#set par(leading: 1em)

// ---------- Configurations ----------
= Raddy Documentation

= Conventions
- If const generics `N`, `L`, `R`, and `C` appear in the same type:
  - `N` is for the dimension of the vector that *the derivative is with respect to*.
  - `L` is the length of the vector.
  - `R` is the number of rows in a matrix.
  - `C` is the number of columns in a matrix.
  - When creating active variables, ensure consistency between dimensions and lengths as required.

= Core Data Structure: `Ad<const N: usize>`
- *Generics*
  - `N`: \# dimension of *variable* (that requires derivatives).
- *Notes*
  - Computes with precision `f64`.
  - Represents both scalar and structured (vector/matrix) types with derivative information.


= Constructors

== `pub fn scalar<const N: usize>(value: f64) -> Ad<N>`
- *Description*
  - Creates an inactive scalar `Ad` value with specified input dimensions.
- *Arguments*
  - `value`: The scalar value to wrap in an `Ad` type.
- *Type Parameters*
  - `N`: The input dimension (for derivatives).
- *Returns*
  - An `Ad<N>` instance representing an inactive scalar value.

== `pub fn vector_from_slice<const N: usize, const L: usize>(values: &[f64]) -> SVector<Ad<N>, L>`
- *Description*
  - Creates a vector of inactive `Ad` values with separate input and vector dimensions.
- *Arguments*
  - `values`: Slice of `f64` values to convert to inactive `Ad` values.
- *Type Parameters*
  - `N`: The input dimension (for derivatives).
  - `L`: The length of the vector.
- *Returns*
  - An `SVector<Ad<N>, L>` where each element is inactive.

== `pub fn vector<const N: usize, const L: usize>(values: &[f64]) -> SVector<Ad<N>, L>`
- *Description*
  - Creates a vector of inactive `Ad` values with separate input and vector dimensions.
- *Arguments*
  - `values`: Slice of `f64` values to convert to inactive `Ad` values.
- *Type Parameters*
  - `N`: The input dimension (for derivatives).
  - `L`: The length of the vector.
- *Returns*
  - An `SVector<Ad<N>, L>` where each element is inactive.
- *Notes*
  - The function name `vectpr` appears to be a typo and functions identically to `vector_from_slice`.

== `pub fn matrix_from_row_slice<const N: usize, const R: usize, const C: usize>(values: &[f64]) -> SMatrix<Ad<N>, R, C>`
- *Description*
  - Creates a matrix of inactive `Ad` values from a row-major slice.
- *Arguments*
  - `values`: Slice of `f64` values in row-major order to convert to inactive `Ad` values.
- *Type Parameters*
  - `N`: The input dimension (for derivatives).
  - `R`: The number of rows.
  - `C`: The number of columns.
- *Panics*
  - If the length of `values` does not equal `R * C`.
- *Returns*
  - An `SMatrix<Ad<N>, R, C>` where each element is inactive.

== `pub fn matrix_from_column_slice<const N: usize, const R: usize, const C: usize>(values: &[f64]) -> SMatrix<Ad<N>, R, C>`
- *Description*
  - Creates a matrix of inactive `Ad` values from a column-major slice.
- *Arguments*
  - `values`: Slice of `f64` values in column-major order to convert to inactive `Ad` values.
- *Type Parameters*
  - `N`: The input dimension (for derivatives).
  - `R`: The number of rows.
  - `C`: The number of columns.
- *Panics*
  - If the length of `values` does not equal `R * C`.
- *Returns*
  - An `SMatrix<Ad<N>, R, C>` where each element is inactive.

== `pub fn matrix<const N: usize, const R: usize, const C: usize>(matrix: SMatrix<f64, R, C>) -> SMatrix<Ad<N>, R, C>`
- *Description*
  - Converts a matrix of `f64` values to a matrix of inactive `Ad` values.
- *Arguments*
  - `matrix`: The matrix of `f64` values to convert.
- *Type Parameters*
  - `N`: The input dimension (for derivatives).
  - `R`: The number of rows.
  - `C`: The number of columns.
- *Returns*
  - An `SMatrix<Ad<N>, R, C>` where each element is inactive.

= Supports
- *Elementary Functions*
  - Supports functions such as `sin`, `cosh`, `exp`, `ln`, etc.
  - *Does not* support `"atan"` by design; use `"atan2"` instead.
- *Matrix Operations*
  - Norms and determinants for matrices.
  - Matrix multiplication.
  - Singular Value Decomposition (SVD), though usage is discouraged due to potential numerical issues.