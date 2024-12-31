# Raddy
Trying to port some portion of [TinyAD](https://github.com/patr-schm/TinyAD) to Rust.

# Usage

## Scalars
```rust
let x = Ad::ad(3.421);
let x = &x; // Please read section: Notes
let y = x.sin() * x + x.ln();
println!("{}", y.grad()[(0, 0)]);
```

## Vectors
The following code depends on `rand` crate.
```rust
let mut rng = thread_rng();
const N_TEST_MAT_4: usize = 4;
type NaConst = Const<N_TEST_MAT_4>;
const N_VEC_4: usize = N_TEST_MAT_4 * N_TEST_MAT_4;

let vals: &[f64] = &(0..N_VEC_4)
    .map(|_| rng.gen_range(-4.0..4.0))
    .collect::<Vec<_>>();

let s: SVector<Ad<N_VEC_4>, N_VEC_4> = Ad::vec(vals);
let z = s
    .clone()
    // This reshape is COL MAJOR!!!!!!!!!!!!!
    .reshape_generic(NaConst {}, NaConst {})
    .transpose();

let det = z.determinant();
println!("{}", y.grad());
println!("{}", y.hess());
```

Please see `src/test` for details.

# Notes
1. `Copy` is **not** implemented for `Ad<N>` types, since its cost is not negligible.
- This reminds you to (in most cases) use a borrow type `&Ad<N>` to call methods on `&Ad<N>`; or to explicitly clone it if the cost is acceptable.

# Progress

- [x] Univariate
  - [x] Gradient
  - [x] Hessian
  - [x] Tests
- [x] Multivariate
  - [x] Gradient
  - [x] Hessian
  - [ ] Tests
    - [x] Norm
    - [x] Determinant
- [ ] Get nalgebra as well as compiler happy
  - [x] Implement `ComplexField` for `&Ad<N>` (Not `Ad<N>`) (a huge task that may involve codegen/metaprogramming...)
  - [ ] Incrementally implement the trait in `ComplexField` if some methods need them
- [ ] An option to allocate hessian on heap
- [ ] `f64` & `Scalar` Interop (How to? Seems sort of impossible due to [orphan rule](https://doc.rust-lang.org/book/ch10-02-traits.html)) (We use the same sort of workaround as `faer`)

## Untested

## Required implementations:
- Norm:
  - most in `ComplexField`

# TODO
1. Figure out how to implement custom scalar type in `nalgebra`. (done)
   1. Implement custom `norm`, do not try to implement `ComplexField` in nalgebra, that's a big mess;
   2. Refer to `testscalar.rs`;
   3. Inactive vector/matrix must be also in Variable type but have no grad/hess. Cannot use base nalgebra type with f64.                         
2. An option to allocate hessian on heap (stack overflow encountered...)


># Notes For Myself
>1. Test code makes use of [Symars](https://github.com/Da1sypetals/Symars), which generates Rust code from SymPy expressions.
>2. When getting numerical bugs, **First check the argument order of symars generated functions**!!!
