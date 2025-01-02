use crate::{make, types::advec, Ad};
use faer::{
    sparse::{CreationError, SparseColMat},
    Col,
};
use itertools::Itertools;

/// N: problem size of a single objective.

pub struct ComputedObjective<const N: usize> {
    pub value: f64,
    pub grad: Col<f64>,
    pub hess_trips: Vec<(usize, usize, f64)>,
}

pub trait Objective<const N: usize> {
    fn eval(&self, variables: &advec<N, N>) -> Ad<N>;

    /// Compute value, grad, hess(triplets) in one go.
    fn compute(&self, x: &Col<f64>, operand_indices: &[[usize; N]]) -> ComputedObjective<N> {
        let mut value = 0.0;
        let mut grad = Col::zeros(x.nrows());
        let mut hess_trips = Vec::new();

        operand_indices.iter().for_each(|&global_inds| {
            let vals = global_inds.map(|i| x[i]);
            let vals_slice = vals.as_slice();
            let vars = make::vec(vals_slice);

            let obj = self.eval(&vars);

            let ind = global_inds.into_iter().enumerate();

            value += obj.value;

            ind.clone()
                .for_each(|(ilocal, iglobal)| grad[iglobal] += obj.grad[ilocal]); // THIS IS += NOT = !!!!!!!!!!!

            // dbg!(&grad);

            ind.clone().cartesian_product(ind).for_each(
                |((ixlocal, ixglobal), (iylocal, iyglobal))| {
                    hess_trips.push((ixglobal, iyglobal, obj.hess[(ixlocal, iylocal)]));
                },
            );
        });

        ComputedObjective {
            value,
            grad,
            hess_trips,
        }
    }

    fn value(&self, x: &Col<f64>, operand_indices: &[[usize; N]]) -> f64 {
        let mut res = 0.0;

        operand_indices.iter().for_each(|&ind| {
            let binding = ind.map(|i| x[i]);
            let values = binding.as_slice();
            let vars: advec<N, N> = make::vec(values);

            let obj = self.eval(&vars);

            res += obj.value;
        });

        res
    }

    fn grad(&self, x: &Col<f64>, operand_indices: &[[usize; N]]) -> Col<f64> {
        let mut res = Col::zeros(x.nrows());

        operand_indices.iter().for_each(|&ind| {
            let binding = ind.map(|i| x[i]);
            let values = binding.as_slice();
            let vars: advec<N, N> = make::vec(values);

            let obj = self.eval(&vars);

            // THIS IS += NOT = !!!!!!!!!!!
            ind.into_iter()
                .enumerate()
                .for_each(|(ilocal, iglobal)| res[iglobal] += obj.grad[ilocal]);
        });

        res
    }

    fn hess_trips(&self, x: &Col<f64>, operand_indices: &[[usize; N]]) -> Vec<(usize, usize, f64)> {
        let mut trips = Vec::new();

        operand_indices.iter().for_each(|&ind| {
            let binding = ind.map(|i| x[i]);
            let values = binding.as_slice();
            let vars: advec<N, N> = make::vec(values);

            let obj = self.eval(&vars);

            let ind = ind.into_iter().enumerate();

            ind.clone().cartesian_product(ind).for_each(
                |((ixlocal, ixglobal), (iylocal, iyglobal))| {
                    trips.push((ixglobal, iyglobal, obj.hess[(ixlocal, iylocal)]));
                },
            );
        });

        trips
    }

    fn hess(
        &self,
        x: &Col<f64>,
        operand_indices: &[[usize; N]],
    ) -> Result<SparseColMat<usize, f64>, CreationError> {
        let n = x.nrows();
        SparseColMat::try_new_from_triplets(n, n, &self.hess_trips(x, operand_indices))
    }
}
