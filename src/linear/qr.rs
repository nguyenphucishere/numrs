use crate::linear::gramschmidt::gramschmidt;
use crate::matrix::Matrix;
use crate::utils::numbers::Numeric;

pub fn qr<N: Numeric>(a: &Matrix<N>) -> (Matrix<N>, Matrix<N>){

    let q = gramschmidt(&a.col_space()).to_matrix();
    let r = q.transpose() * a;

    (q, r)
}