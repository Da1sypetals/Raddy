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
   1. Implement custom `norm`, do not try to implement `ComplexField` in nalgebra;
   2. Refer to `testscalar.rs`;