# Raddy
Trying to port some portion of [TinyAD](https://github.com/patr-schm/TinyAD) to Rust.

# Progress

- [x] Univariate
  - [x] Gradient
  - [x] Hessian
  - [x] Tests
- [x] Multivariate
  - [x] Gradient
  - [x] Hessian
  - [ ] Tests
- [ ] An option to allocate hessian on heap
- [ ] `f64` & `Scalar` Interop (How to? Seems sort of impossible due to [orphan rule](https://doc.rust-lang.org/book/ch10-02-traits.html))

# TODO
1. Figure out how to implement custom scalar type in `nalgebra`. (done)
   1. Implement custom `norm`, do not try to implement `ComplexField` in nalgebra, that's a big mess;
   2. Refer to `testscalar.rs`;
   3. Inactive vector/matrix must be also in Variable type but have no grad/hess. Cannot use base nalgebra type with f64.                         
2. An option to allocate hessian on heap (stack overflow encountered...)

# Notes
Test code makes use of [Symars](https://github.com/Da1sypetals/Symars), which generates Rust code from SymPy expressions.