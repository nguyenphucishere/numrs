use crate::linear::eig::eig;
use crate::matrix::Matrix;
use crate::utils::numbers::Numeric;

pub fn svd<N: Numeric>(A: &Matrix<N>) -> (Matrix<N>, Matrix<N>, Matrix<N>) {
    let (m, n) = A.shape();
    // A = U * Sigma * V^T
    //A^TA = V * Sigma^2 * V^T

    // AV Sigma^-1 = U

    let AtA = A.transpose() * A;
    let (Sigma_squared, V) = eig(&AtA, None);
    let (a, b) = Sigma_squared.shape();
    
    let mut Sigma: Matrix<N> = Sigma_squared.map(|x| x.sqrt().ground_if_zero()).collect();
    Sigma.reshape(a, b);

    let mut U = A * &V * Sigma.safe_inverse();
    U.dim_truncate(m, m);
    
    //more accurate but slower
    // let AAt = A * A.transpose();
    // let U = eig(&AAt, None).1;
    
    // V.shape().0 = n
    Sigma.dim_truncate(m, n);

    (U, Sigma, V)
}