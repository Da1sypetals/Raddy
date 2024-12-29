use crate::Scalar;
use std::cmp::Ordering;

impl<const N: usize> PartialEq for Scalar<N> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<const N: usize> PartialOrd for Scalar<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<const N: usize> PartialEq<f64> for Scalar<N> {
    fn eq(&self, other: &f64) -> bool {
        self.value == *other
    }
}

impl<const N: usize> PartialOrd<f64> for Scalar<N> {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl<const N: usize> PartialEq<Scalar<N>> for f64 {
    fn eq(&self, other: &Scalar<N>) -> bool {
        *self == other.value
    }
}

impl<const N: usize> PartialOrd<Scalar<N>> for f64 {
    fn partial_cmp(&self, other: &Scalar<N>) -> Option<Ordering> {
        self.partial_cmp(&other.value)
    }
}
