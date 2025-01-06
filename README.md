# Raddy
Trying to port some portion of [TinyAD](https://github.com/patr-schm/TinyAD) to Rust.

# Usage

First add to your `Cargo.toml`:
```toml
raddy-ad = "*"
```
>_Sadly the name `raddy` is occupied by a non-maintaining crate whose owner does not seem to want to negotiate with me. However the lib name (the one you import in your `.rs` code) is still `raddy`_ .

## Scalars
```rust
use raddy::make::var;

fn main() {
    let val = 1.14;
    let var = var::scalar(val);
    let var = &var;
    let y = var.sin() * var + var.ln();
    let grad = val * val.cos() + val.sin() + val.recip();
    let hess = -val * val.sin() + 2.0 * val.cos() - val.powi(-2);
    dbg!((y.grad()[(0, 0)] - grad).abs());
    dbg!((y.hess()[(0, 0)] - hess).abs());
}
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

let s: SVector<Ad<N_VEC_4>, N_VEC_4> = var::vector(vals);
let z = s
    .clone()
    // This reshape is COL MAJOR!!!!!!!!!!!!!
    .reshape_generic(NaConst {}, NaConst {})
    .transpose();

let det = z.determinant();
println!("{}", y.grad());
println!("{}", y.hess());
```

## Sparse
1. First define your per-element objective:
```rust
struct SpringEnergy {
    k: f64,
    restlen: f64,
}

// 2d * 2nodes = 4dof
impl ObjectiveFunction<4> for SpringEnergy {
    fn eval(&self, variables: &raddy::types::advec<4, 4>) -> raddy::Ad<4> {
        let p1 = advec::<4, 2>::new(variables[0].clone(), variables[1].clone());
        let p2 = advec::<4, 2>::new(variables[2].clone(), variables[3].clone());

        let len = (p2 - p1).norm();
        // Hooke's law
        let potential = val::scalar(0.5 * self.k) * (len - val::scalar(self.restlen)).powi(2);

        potential
    }
}


```
2. Then, define your elements (indices, springs here):
```rust
let springs = vec![[0, 1, 2, 3], [2, 3, 4, 5], [0, 1, 4, 5]];
let x0 = faer::col::from_slice(&[0.0, 0.0, 2.0, 0.0, 1.0, 2.0]).to_owned();

let obj = SpringEnergy {
    k: 10000.0,
    restlen: 1.0,
};
```
3. Finally, compute:
```rust
let computed: ComputedObjective<4> = obj.compute(&x0, &springs);

/*
pub struct ComputedObjective<const N: usize> {
    pub value: f64,
    pub grad: Col<f64>,
    pub hess_trips: Vec<(usize, usize, f64)>,
}
*/
```

Please see `src/examples` and `src/test` for details.

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
    - [x] Matmul
- [ ] Get nalgebra as well as compiler happy
  - [x] Implement `ComplexField` for `&Ad<N>` (Not `Ad<N>`) (a huge task that may involve codegen/metaprogramming...)
  - [ ] Incrementally implement the trait in `ComplexField` if some methods need them
- [ ] Sparse interface
  - [x] Define sparse problem (generic on problem size)
  - [x] Compute sparse problem
  - [ ] Test
    - [x] Mass spring: grad/hess
    - [x] Mass spring: results
    - [ ] Neo Hookean
  - [x] Make an example: [mass-spring system](https://github.com/Da1sypetals/Raddy-examples)
- [ ] An option to allocate hessian on heap
- [ ] `f64` & `Scalar` Interop (How to? Seems sort of impossible due to [orphan rule](https://doc.rust-lang.org/book/ch10-02-traits.html)) (We use the same sort of workaround as `faer`)


># Notes For Myself
>1. Test code makes use of [Symars](https://github.com/Da1sypetals/Symars), which generates Rust code from SymPy expressions.
>2. When getting numerical bugs, **First check the argument order of symars generated functions**!!!
