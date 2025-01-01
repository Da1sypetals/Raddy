extern crate nalgebra as na;

pub mod compare;
pub mod ctor;
pub mod matrix;
pub mod misc;
/// Please Note that all `unimplemented!` methods are not intended for use.
/// If any operation encountered these, please raise an issue.
pub mod scalar;
pub mod sparse;
#[cfg(test)]
mod test;
pub mod types;

// ######################################################################################
// ################################### Implementation ###################################
// ######################################################################################

use nalgebra::{SMatrix, SVector};
use types::{mat, vec};

// ################################### Data Structure ###################################

#[derive(Debug, Clone)]
pub struct Ad<const N: usize> {
    pub(crate) value: f64,
    pub(crate) grad: vec<N>,
    pub(crate) hess: mat<N>,
}

// ################################### Accessors ###################################

impl<const N: usize> Ad<N> {
    /// Makes all-zeroed Scalar. Only used internally.

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn grad(&self) -> vec<N> {
        self.grad.clone()
    }

    pub fn hess(&self) -> mat<N> {
        self.hess.clone()
    }
}

pub trait GetValue<const R: usize, const C: usize> {
    type Value;
    fn value(&self) -> Self::Value;
}

impl<const N: usize, const R: usize, const C: usize> GetValue<R, C> for SMatrix<Ad<N>, R, C> {
    type Value = SMatrix<f64, R, C>;
    fn value(&self) -> Self::Value {
        let mut val = Self::Value::zeros();
        for r in 0..R {
            for c in 0..C {
                val[(r, c)] = self[(r, c)].value;
            }
        }
        val
    }
}

// ################################### Public Constructors ###################################

impl Ad<1> {
    pub fn given_scalar(value: f64, grad: f64, hess: f64) -> Self {
        Self {
            value,
            grad: vec::from_row_slice(&[grad]),
            hess: mat::from_row_slice(&[hess]),
        }
    }

    pub fn active_scalar(value: f64) -> Self {
        let mut res = Self::_zeroed();

        res.value = value;
        res.grad[0] = 1.0;

        res
    }
}

impl<const N: usize> Ad<N> {
    pub fn inactive_scalar(value: f64) -> Self {
        let mut res = Self::_zeroed();
        res.value = value;
        res
    }

    pub fn inactive_vector(values: &SVector<f64, N>) -> SVector<Self, N> {
        let mut scalars = Vec::new();
        for i in 0..N {
            let scalar = Self::inactive_scalar(values[i]);
            scalars.push(scalar);
        }

        SVector::from_column_slice(&scalars)
    }

    pub fn inactive_from_slice(values: &[f64]) -> SVector<Self, N> {
        assert_eq!(
            values.len(),
            N,
            "Slice length mismatch: expected {}, got {}",
            N,
            values.len()
        );
        Self::inactive_vector(&SVector::from_column_slice(values))
    }

    pub fn given_vector(value: f64, grad: &vec<N>, hess: &mat<N>) -> Self {
        Self {
            value,
            grad: grad.clone(),
            hess: hess.clone(),
        }
    }

    pub fn active_vector(values: &SVector<f64, N>) -> SVector<Self, N> {
        let mut scalars = Vec::new();
        for i in 0..N {
            let scalar = Self::_active_scalar_with_index(values[i], i);
            scalars.push(scalar);
        }

        SVector::from_column_slice(&scalars)
    }

    pub fn active_from_slice(values: &[f64]) -> SVector<Self, N> {
        assert_eq!(
            values.len(),
            N,
            "Slice length mismatch: expected {}, got {}",
            N,
            values.len()
        );
        Self::active_vector(&SVector::from_column_slice(values))
    }
}

// ################################### Private Constructors ###################################

impl<const N: usize> Ad<N> {
    fn _active_scalar_with_index(value: f64, index: usize) -> Self {
        let mut res = Self::_zeroed();

        res.value = value;
        res.grad[index] = 1.0;

        res
    }

    fn _zeroed() -> Self {
        Self {
            value: 0.0,
            grad: vec::zeros(),
            hess: mat::zeros(),
        }
    }
}

// ################################### Utils ###################################

impl<const N: usize> Ad<N> {
    fn chain(
        value: f64, // f
        d: f64,     // df/da
        d2: f64,    // ddf/daa
        a: &Self,
    ) -> Self {
        let mut res = Self::_zeroed();

        res.value = value;
        res.grad = d * &a.grad;
        res.hess = d2 * &a.grad * &a.grad.transpose() + d * a.hess;

        res
    }
}
