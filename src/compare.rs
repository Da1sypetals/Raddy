use crate::Ad;
use std::cmp::Ordering;

impl<const N: usize> PartialEq for Ad<N> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<const N: usize> PartialOrd for Ad<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<const N: usize> PartialEq<f64> for Ad<N> {
    fn eq(&self, other: &f64) -> bool {
        self.value == *other
    }
}

impl<const N: usize> PartialOrd<f64> for Ad<N> {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl<const N: usize> PartialEq<Ad<N>> for f64 {
    fn eq(&self, other: &Ad<N>) -> bool {
        *self == other.value
    }
}

impl<const N: usize> PartialOrd<Ad<N>> for f64 {
    fn partial_cmp(&self, other: &Ad<N>) -> Option<Ordering> {
        self.partial_cmp(&other.value)
    }
}
