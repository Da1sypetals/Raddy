use super::function::ObjectiveFunction;
use crate::{ctor, types::advec, Ad};
use faer::{
    sparse::{CreationError, SparseColMat},
    Col,
};
use itertools::Itertools;

/// N: problem size of a single objective.
pub struct Objective<const N: usize, FuncType: ObjectiveFunction<N>> {
    operand_indices: Vec<[usize; N]>,
    func: FuncType,
}

// impl<const N: usize> Sum for Ad<N> {
//     fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
//         let mut res = Ad::<N>::zero();
//         for x in iter {
//             res += x;
//         }
//         res
//     }
// }

impl<const N: usize, FuncType: ObjectiveFunction<N>> Objective<N, FuncType> {
    pub fn new(func: FuncType, operand_indices: Vec<[usize; N]>) -> Self {
        Self {
            operand_indices,
            func,
        }
    }
}

impl<const N: usize, FuncType: ObjectiveFunction<N>> Objective<N, FuncType> {
    pub fn value(&self, x: &Col<f64>) -> f64 {
        let mut res = 0.0;

        self.operand_indices.iter().for_each(|&ind| {
            let binding = ind.map(|i| x[i]);
            let values = binding.as_slice();
            let vars: advec<N, N> = ctor::vec(values);

            let obj = self.func.eval(&vars);

            res += obj.value();
        });

        res
    }

    pub fn grad(&self, x: &Col<f64>) -> Col<f64> {
        let mut res = Col::zeros(x.nrows());

        self.operand_indices.iter().for_each(|&ind| {
            let binding = ind.map(|i| x[i]);
            let values = binding.as_slice();
            let vars: advec<N, N> = ctor::vec(values);

            let obj = self.func.eval(&vars);

            ind.into_iter()
                .enumerate()
                .for_each(|(ilocal, iglobal)| res[iglobal] = obj.grad[ilocal]);
        });

        res
    }

    pub fn hess_trips(&self, x: &Col<f64>) -> Vec<(usize, usize, f64)> {
        let mut trips = Vec::new();

        self.operand_indices.iter().for_each(|&ind| {
            let binding = ind.map(|i| x[i]);
            let values = binding.as_slice();
            let vars: advec<N, N> = ctor::vec(values);

            let obj = self.func.eval(&vars);

            let ind = ind.into_iter().enumerate();

            ind.clone().cartesian_product(ind).for_each(
                |((ixlocal, ixglobal), (iylocal, iyglobal))| {
                    trips.push((ixglobal, iyglobal, obj.hess()[(ixlocal, iylocal)]));
                },
            );
        });

        trips
    }

    pub fn hess(&self, x: &Col<f64>) -> Result<SparseColMat<usize, f64>, CreationError> {
        let n = x.nrows();
        SparseColMat::try_new_from_triplets(n, n, &self.hess_trips(x))
    }
}
