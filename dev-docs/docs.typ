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
   lang: "zh", region: "cn",
  size: 12pt,
)

#set text(top-edge: 0.7em, bottom-edge: -0.3em)
#set par(leading: 1em)

// ---------- Configurations ----------
= Raddy Documentation

#v(3%)

= `Ad<const N: usize>`
- Generics
  - `N`: \# dimension of *variable* (that requires derivatives).
- Notes
  - Computes on precision `f64`.

#set heading(numbering: "1.")

= Constructors

== `Ad<1>` (One dimensional)
+ `pub fn active_scalar(value: f64) -> Self `
+ `pub fn given_scalar(value: f64, grad: f64, hess: f64) -> Self`

== `Ad<N>` 
- Note that `SVector<T, N>` is `nalgebra` static column vector type that holds `N` elements.
+ `pub fn inactive_scalar(value: f64) -> Self`
+ `pub fn inactive_vector(values: &SVector<f64, N>) -> SVector<Self, N> `
+ `pub fn inactive_from_slice(values: &[f64]) -> SVector<Self, N>`
+ `pub fn given_vector(value: f64, grad: &vec<N>, hess: &mat<N>) -> Self `
+ `pub fn active_vector(values: &SVector<f64, N>) -> SVector<Self, N>`
+ `pub fn active_from_slice(values: &[f64]) -> SVector<Self, N>`


= Methods
== `fn scale(&self, factor: f64) -> Self`
- Scale to a factor.

== Norms 
- Does not support `nalgebra` norms;
- Methods are listed as follows:
```rust
type Scalar = Ad<N>;
fn l1_norm(&self) -> Self::Scalar;
fn l2_norm(&self) -> Self::Scalar;
fn l2_norm_squared(&self) -> Self::Scalar;
fn lk_norm(&self, k: u32) -> Self::Scalar;
fn linf_norm(&self) -> Self::Scalar;
```

== `fn determinant(&self) -> Self::Scalar`
- Supports determinants for $N<=6$.
  - Too large $N$ may blow the stack.

== Others
- Feel free to call any methods that compiler is happy with, such as `apply()`.
- If you encounter currently not implemented methods that you think should be implemented, please raise an issue at #link("https://github.com/Da1sypetals/Raddy/issues", "Raddy Link").

= Usage
- Please refer to `src/test` for examples.