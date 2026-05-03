use std::ops::{Mul, RangeFull, Index};
use crate::utils::numbers::Numeric;
use crate::matrix::Matrix;

pub struct Scalar<N>{
    val: N,
}

impl<N: Numeric> Scalar<N>{
    pub fn new(val: N) -> Self{
        Scalar { val }
    }
}


impl<N: Numeric> Mul<&Matrix<N>> for &Scalar<N>{
    type Output = Matrix<N>;

    fn mul(self, other: &Matrix<N>) -> Matrix<N>{
        let (rows, cols) = other.shape();

        Matrix::<N>::from_arr(&other[..].iter().map(|&x| (x * self.val).ground_if_zero()).collect::<Vec<_>>(), rows, cols)
    }
}

impl<N: Numeric> Mul<&Scalar<N>> for &Matrix<N>{
    type Output = Matrix<N>;

    fn mul(self, other: &Scalar<N>) -> Matrix<N>{
        let (rows, cols) = self.shape();

        Matrix::<N>::from_arr(&self[..].iter().map(|&x| (x * other.val).ground_if_zero()).collect::<Vec<_>>(), rows, cols)
    }
}

impl<N: Numeric> Index<RangeFull> for Scalar<N>{
    type Output = N;
    
    fn index(&self, _index: RangeFull) -> &N{
        &self.val
    }
}