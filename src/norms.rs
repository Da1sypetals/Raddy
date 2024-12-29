use nalgebra::SVector;

use crate::Variable;

pub trait LkNorm {
    type Scalar;
    fn l1_norm(&self) -> Self::Scalar;
    fn l2_norm(&self) -> Self::Scalar;
    fn l2_norm_squared(&self) -> Self::Scalar;
    fn lk_norm(&self, k: u32) -> Self::Scalar;
    fn linf_norm(&self) -> Self::Scalar;
}

impl<const N: usize> LkNorm for SVector<Variable<N>, N> {
    type Scalar = Variable<N>;

    fn l1_norm(&self) -> Self::Scalar {
        let mut res = Variable::_zeroed();
        for i in 0..N {
            res += self[i].clone();
        }
        res
    }

    fn l2_norm(&self) -> Self::Scalar {
        let mut res = Variable::_zeroed();
        for i in 0..N {
            res += self[i].square();
        }
        res.sqrt()
    }

    fn l2_norm_squared(&self) -> Self::Scalar {
        let mut res = Variable::_zeroed();
        for i in 0..N {
            res += self[i].square();
        }
        res
    }

    fn lk_norm(&self, k: u32) -> Self::Scalar {
        let mut res = Variable::_zeroed();
        for i in 0..N {
            res += self[i].powi(k as i32);
        }
        res.powf(1.0 / (k as f64))
    }

    fn linf_norm(&self) -> Self::Scalar {
        let mut res = self[0].clone();
        for i in 1..N {
            res = res.max(&self[i].clone());
        }
        res
    }
}
