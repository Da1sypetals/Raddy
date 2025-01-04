use crate::{make::var, types::advec, Ad};
use faer::{
    sparse::{CreationError, SparseColMat},
    Col,
};
use itertools::Itertools;

/// Represents the computed results of an objective function evaluation
/// including the function value, gradient, and Hessian triplets.
///
/// ## Type Parameters
/// - `N`: The problem size/dimension of a single objective
///
/// ## Fields
/// - `value`: The computed objective function value
/// - `grad`: The gradient vector (first derivatives)
/// - `hess_trips`: Hessian matrix entries stored as (row, col, value) triplets
pub struct ComputedObjective<const N: usize> {
    pub value: f64,
    pub grad: Col<f64>,
    pub hess_trips: Vec<(usize, usize, f64)>,
}

/// Defines the interface for sparse objective functions
///
/// ## Type Parameters
/// - `N`: The problem size/dimension of a single objective
///
/// ## Associated Types
/// - `EvalArgs`: Additional arguments needed for objective evaluation
///
/// ## Example
/// ```ignore
/// struct MyObjective;
///
/// impl Objective<3> for MyObjective {
///     type EvalArgs = f64; // Example args type
///     
///     fn eval(&self, variables: &advec<3, 3>, args: &f64) -> Ad<3> {
///         todo!("Your implementation")
///     }
/// }
/// ```
pub trait Objective<const N: usize> {
    type EvalArgs;

    /// Evaluates the objective function for given variables
    ///
    /// ## Arguments
    /// - `variables`: The input variables as an advec
    /// - `args`: Additional evaluation arguments
    ///
    /// ## Returns
    /// An `Ad<N>` containing the function value, gradient and Hessian
    fn eval(&self, variables: &advec<N, N>, args: &Self::EvalArgs) -> Ad<N>;

    /// Helper method to evaluate objective for given indices
    ///
    /// ## Arguments
    /// - `global_inds`: Global indices of variables to evaluate
    /// - `x`: The full variable vector
    /// - `args`: Additional evaluation arguments
    ///
    /// ## Returns
    /// An `Ad<N>` containing the local evaluation results
    fn evaluate_for_indices(
        &self,
        global_inds: [usize; N],
        x: &Col<f64>,
        args: &Self::EvalArgs,
    ) -> Ad<N> {
        let vals = global_inds.map(|i| x[i]);
        let vals_slice = vals.as_slice();
        let vars = var::vector_from_slice(vals_slice);
        self.eval(&vars, args)
    }

    /// Computes value, gradient and Hessian triplets in one operation
    ///
    /// ## Arguments
    /// - `x`: The full variable vector, may be large
    /// - `operand_indices`: Slice of indices of variables to evaluate
    /// - `args`: Additional evaluation arguments
    ///
    /// ## Returns
    /// A `ComputedObjective<N>` containing all computed results
    fn compute(
        &self,
        x: &Col<f64>,
        operand_indices: &[[usize; N]],
        args: &Self::EvalArgs,
    ) -> ComputedObjective<N> {
        let mut value = 0.0;
        let mut grad = Col::zeros(x.nrows());
        let mut hess_trips = Vec::new();

        for &global_inds in operand_indices {
            let obj = self.evaluate_for_indices(global_inds, x, args);

            let ind = global_inds.into_iter().enumerate();

            value += obj.value;

            ind.clone()
                .for_each(|(ilocal, iglobal)| grad[iglobal] += obj.grad[ilocal]);

            ind.clone().cartesian_product(ind).for_each(
                |((ixlocal, ixglobal), (iylocal, iyglobal))| {
                    hess_trips.push((ixglobal, iyglobal, obj.hess[(ixlocal, iylocal)]));
                },
            );
        }

        ComputedObjective {
            value,
            grad,
            hess_trips,
        }
    }

    /// Computes just the objective function value
    ///
    /// ## Arguments
    /// - `x`: The full variable vector
    /// - `operand_indices`: Slice of indices of variables to evaluate
    /// - `args`: Additional evaluation arguments
    ///
    /// ## Returns
    /// The computed objective function value
    fn value(&self, x: &Col<f64>, operand_indices: &[[usize; N]], args: &Self::EvalArgs) -> f64 {
        let mut res = 0.0;

        operand_indices.iter().for_each(|&ind| {
            let obj = self.evaluate_for_indices(ind, x, args);
            res += obj.value;
        });

        res
    }

    /// Computes just the gradient vector
    ///
    /// ## Arguments
    /// - `x`: The full variable vector
    /// - `operand_indices`: Slice of indices of variables to evaluate
    /// - `args`: Additional evaluation arguments
    ///
    /// ## Returns
    /// The computed gradient vector
    fn grad(
        &self,
        x: &Col<f64>,
        operand_indices: &[[usize; N]],
        args: &Self::EvalArgs,
    ) -> Col<f64> {
        let mut res = Col::zeros(x.nrows());

        operand_indices.iter().for_each(|&ind| {
            let obj = self.evaluate_for_indices(ind, x, args);
            ind.into_iter()
                .enumerate()
                .for_each(|(ilocal, iglobal)| res[iglobal] += obj.grad[ilocal]);
        });

        res
    }

    /// Computes Hessian matrix entries as triplets
    ///
    /// ## Arguments
    /// - `x`: The full variable vector
    /// - `operand_indices`: Slice of indices of variables to evaluate
    /// - `args`: Additional evaluation arguments
    ///
    /// ## Returns
    /// Vector of (row, col, value) triplets representing the Hessian matrix
    fn hess_trips(
        &self,
        x: &Col<f64>,
        operand_indices: &[[usize; N]],
        args: &Self::EvalArgs,
    ) -> Vec<(usize, usize, f64)> {
        let mut trips = Vec::new();

        operand_indices.iter().for_each(|&ind| {
            let obj = self.evaluate_for_indices(ind, x, args);
            let ind = ind.into_iter().enumerate();

            ind.clone().cartesian_product(ind).for_each(
                |((ixlocal, ixglobal), (iylocal, iyglobal))| {
                    trips.push((ixglobal, iyglobal, obj.hess[(ixlocal, iylocal)]));
                },
            );
        });

        trips
    }

    /// Computes the Hessian matrix as a sparse matrix
    ///
    /// ## Arguments
    /// - `x`: The full variable vector
    /// - `operand_indices`: Slice of indices of variables to evaluate
    /// - `args`: Additional evaluation arguments
    ///
    /// ## Returns
    /// A sparse matrix representation of the Hessian
    fn hess(
        &self,
        x: &Col<f64>,
        operand_indices: &[[usize; N]],
        args: &Self::EvalArgs,
    ) -> Result<SparseColMat<usize, f64>, CreationError> {
        let n = x.nrows();
        SparseColMat::try_new_from_triplets(n, n, &self.hess_trips(x, operand_indices, args))
    }
}
