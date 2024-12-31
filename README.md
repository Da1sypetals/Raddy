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
- [ ] Get nalgebra as well as compiler happy
  - [x] Implement `ComplexField` for `&Ad<N>` (Not `Ad<N>`) (a huge task that may involve codegen/metaprogramming...)
  - [ ] Incrementally implement the trait in `ComplexField` if some methods need them
- [ ] An option to allocate hessian on heap
- [ ] `f64` & `Scalar` Interop (How to? Seems sort of impossible due to [orphan rule](https://doc.rust-lang.org/book/ch10-02-traits.html)) (We use the same sort of workaround as `faer`)

## Untested

### Univariate
- [ ] `f64.pow(Ad)`
- [ ] `f64.pow(Ad)`
- [ ] `f64.pow(Ad)`

## Required implementations:
- Norm:
  - most in `ComplexField`

# TODO
1. Figure out how to implement custom scalar type in `nalgebra`. (done)
   1. Implement custom `norm`, do not try to implement `ComplexField` in nalgebra, that's a big mess;
   2. Refer to `testscalar.rs`;
   3. Inactive vector/matrix must be also in Variable type but have no grad/hess. Cannot use base nalgebra type with f64.                         
2. An option to allocate hessian on heap (stack overflow encountered...)

# Notes
1. Test code makes use of [Symars](https://github.com/Da1sypetals/Symars), which generates Rust code from SymPy expressions.
2. When getting numerical bugs, **First check the argument order of symars generated functions**!!!
