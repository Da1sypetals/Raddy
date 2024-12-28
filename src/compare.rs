use crate::Variable;
use std::cmp::Ordering;

impl<const N: usize> PartialEq for Variable<N> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<const N: usize> PartialOrd for Variable<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<const N: usize> PartialEq<f64> for Variable<N> {
    fn eq(&self, other: &f64) -> bool {
        self.value == *other
    }
}

impl<const N: usize> PartialOrd<f64> for Variable<N> {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl<const N: usize> PartialEq<Variable<N>> for f64 {
    fn eq(&self, other: &Variable<N>) -> bool {
        *self == other.value
    }
}

impl<const N: usize> PartialOrd<Variable<N>> for f64 {
    fn partial_cmp(&self, other: &Variable<N>) -> Option<Ordering> {
        self.partial_cmp(&other.value)
    }
}
