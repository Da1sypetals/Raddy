# Raddy
Trying to port some portion of [TinyAD](https://github.com/patr-schm/TinyAD) to Rust.

# Progress

- [x] Univariate
  - [x] Gradient
  - [x] Hessian
- [ ] Multivariate
  - [ ] Gradient
  - [ ] Hessian

# TODO
1. Figure out how to implement custom scalar type in `nalgebra`.
   1. Implement custom `norm`, do not try to implement `ComplexField` in nalgebra, that's a big mess;
   2. Refer to `testscalar.rs`;
   3. Inactive vector/matrix must be also in Variable type but have no grad/hess. Cannot use base nalgebra type with f64.                         