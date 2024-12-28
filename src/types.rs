#![allow(non_camel_case_types)]

pub type vec<const N: usize> = nalgebra::SVector<f64, N>;
pub type mat<const N: usize> = nalgebra::SMatrix<f64, N, N>;
