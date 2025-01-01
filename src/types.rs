#![allow(non_camel_case_types)]

use crate::Ad;

pub type vec<const L: usize> = na::SVector<f64, L>;
pub type mat<const RC: usize> = na::SMatrix<f64, RC, RC>;

/// N is the variable size the vector is w.r.t.
pub type advec<const N: usize, const L: usize> = na::SVector<Ad<N>, L>;
/// N is the variable size the matrix is w.r.t.
pub type admat<const N: usize, const R: usize, const C: usize> = na::SMatrix<Ad<N>, R, C>;
