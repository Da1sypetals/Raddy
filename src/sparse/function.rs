use crate::{types::advec, Ad};

pub trait ObjectiveFunction<const N: usize> {
    fn eval(&self, variables: &advec<N, N>) -> Ad<N>;
}
