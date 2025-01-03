#![doc = include_str!("../README.md")]
extern crate nalgebra as na;

/// Comparison operations and utilities for AD values.
pub mod compare;

/// Factory functions for creating AD values and vectors.
pub mod make;

/// Matrix operations, decompositions, and specialized routines.
pub mod matrix;

/// Miscellaneous utilities and experimental features.
mod misc;

/// Scalar operations, operator traits, and field implementations.
/// Please Note that all `unimplemented!` methods are not intended for use.
/// If any operation encountered these, please raise an issue.
pub mod scalar;

/// Sparse matrix differentiation functionalities.
pub mod sparse;

/// Unit tests and validation code.
#[cfg(test)]
mod test;

/// Core type definitions and AD value implementations.
pub mod types;

// ######################################################################################
// ################################### Implementation ###################################
// ######################################################################################

use nalgebra::{SMatrix, SVector};
use types::{mat, vec};

// ################################### Data Structure ###################################

/// Automatic differentiation value tracking first and second derivatives
///
/// # Type Parameters
/// * `N` - The dimension of the input space (number of variables)
///
/// # Fields (private)
/// * `value` - The current value of the function
/// * `grad` - The gradient (first derivatives) as a vector
/// * `hess` - The Hessian matrix (second derivatives)
#[derive(Debug, Clone)]
pub struct Ad<const N: usize> {
    pub(crate) value: f64,
    pub(crate) grad: vec<N>,
    pub(crate) hess: mat<N>,
}

// ################################### Accessors ###################################

impl<const N: usize> Ad<N> {
    /// Returns the current value of the AD variable
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Returns the gradient (first derivatives) of the AD variable
    ///
    /// # Returns
    /// The gradient (A vector containing the partial derivatives with respect to each input variable).
    pub fn grad(&self) -> vec<N> {
        self.grad.clone()
    }

    /// Returns the Hessian matrix (second derivatives) of the AD variable
    ///
    /// # Returns
    /// The [hessian](https://en.wikipedia.org/wiki/Hessian_matrix).
    pub fn hess(&self) -> mat<N> {
        self.hess.clone()
    }
}

/// Trait for extracting numerical values from AD matrices
///
/// # Type Parameters
/// * `R` - Number of rows
/// * `C` - Number of columns
pub trait GetValue<const R: usize, const C: usize> {
    /// The type of the extracted numerical values
    type Value;

    /// Extracts the numerical values from an AD matrix
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
    /// Creates an AD scalar with explicitly specified value, gradient and Hessian
    ///
    /// # Arguments
    /// * `value` - The scalar value
    /// * `grad` - The gradient (first derivative)
    /// * `hess` - The Hessian (second derivative)
    ///
    /// # Returns
    /// A new Ad<1> instance with the specified properties
    pub fn given_scalar(value: f64, grad: f64, hess: f64) -> Self {
        Self {
            value,
            grad: vec::from_row_slice(&[grad]),
            hess: mat::from_row_slice(&[hess]),
        }
    }

    /// Creates an active scalar AD value with unit gradient
    ///
    /// # Arguments
    /// * `value` - The scalar value
    ///
    /// # Returns
    /// A new Ad<1> instance that is active (gradient = 1.0)
    pub fn active_scalar(value: f64) -> Self {
        let mut res = Self::_zeroed();

        res.value = value;
        res.grad[0] = 1.0;

        res
    }
}

impl<const N: usize> Ad<N> {
    /// Creates an inactive scalar AD value with zero gradient and Hessian
    ///
    /// # Arguments
    /// * `value` - The scalar value
    ///
    /// # Returns
    /// A new `Ad<N>` instance that is inactive (gradient = 0)
    pub fn inactive_scalar(value: f64) -> Self {
        let mut res = Self::_zeroed();
        res.value = value;
        res
    }

    /// Creates a vector of inactive AD values from a vector of f64 values
    ///
    /// # Arguments
    /// * `values` - Input vector of numerical values
    ///
    /// # Type Parameters
    /// * `L` - Length of the output vector
    ///
    /// # Returns
    /// A vector of inactive AD values
    pub fn inactive_vector<const L: usize>(values: &SVector<f64, N>) -> SVector<Self, L> {
        let mut scalars = Vec::new();
        for i in 0..N {
            let scalar = Self::inactive_scalar(values[i]);
            scalars.push(scalar);
        }

        SVector::from_column_slice(&scalars)
    }

    /// Creates a vector of inactive AD values from a slice of f64 values
    ///
    /// # Arguments
    /// * `values` - Slice of numerical values
    ///
    /// # Type Parameters
    /// * `L` - Length of the output vector
    ///
    /// # Returns
    /// A vector of inactive AD values
    ///
    /// # Panics
    /// If the slice length doesn't match the input dimension N
    pub fn inactive_from_slice<const L: usize>(values: &[f64]) -> SVector<Self, L> {
        assert_eq!(
            values.len(),
            N,
            "Slice length mismatch: expected {}, got {}",
            N,
            values.len()
        );
        Self::inactive_vector(&SVector::from_column_slice(values))
    }

    /// Creates an AD value with explicitly specified value, gradient and Hessian
    ///
    /// # Arguments
    /// * `value` - The scalar value
    /// * `grad` - The gradient vector
    /// * `hess` - The Hessian matrix
    ///
    /// # Returns
    /// A new `Ad<N>` instance with the specified properties
    pub fn given_vector(value: f64, grad: &vec<N>, hess: &mat<N>) -> Self {
        Self {
            value,
            grad: grad.clone(),
            hess: hess.clone(),
        }
    }

    /// Creates a vector of active AD values from a vector of f64 values
    ///
    /// # Arguments
    /// * `values` - Input vector of numerical values
    ///
    /// # Returns
    /// A vector of active AD values where each element has unit gradient
    /// in its corresponding dimension
    pub fn active_vector(values: &SVector<f64, N>) -> SVector<Self, N> {
        let mut scalars = Vec::new();
        for i in 0..N {
            let scalar = Self::_active_scalar_with_index(values[i], i);
            scalars.push(scalar);
        }

        SVector::from_column_slice(&scalars)
    }

    /// Creates a vector of active AD values from a slice of f64 values
    ///
    /// # Arguments
    /// * `values` - Slice of numerical values
    ///
    /// # Returns
    /// A vector of active AD values
    ///
    /// # Panics
    /// If the slice length doesn't match the input dimension N
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
