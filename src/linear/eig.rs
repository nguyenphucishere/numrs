use crate::matrix::Matrix;
use crate::utils::numbers::Numeric;
use crate::linear::qr::qr;

pub fn eig<N: Numeric>(A: &Matrix<N>, iterations: Option<usize>) -> (Matrix<N>, Matrix<N>) {
    let n = A.shape().0;
    let mut eigenvectors = Matrix::identity(n);
    let mut eigenvalues = A.clone();

    for _ in 0..iterations.unwrap_or(1000) {
        let (Q, R) = qr(&eigenvalues);
        eigenvalues = &R * &Q;
        eigenvectors = &eigenvectors * &Q;
    }

    (eigenvalues, eigenvectors)
}