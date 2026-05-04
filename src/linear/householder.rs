use crate::matrix::Matrix;
use crate::vector::Vector;
use crate::utils::numbers::Numeric;

pub fn householder<N: Numeric>(A: &Vector<N>) -> Matrix<N> {
    println!("Householder vector running");
    let H = Matrix::<N>::identity(A[..].len());
    let mut v = A.clone();

    println!("A sq norm: {}", A.sq_norm());

    v[0] = v[0] + A[0].sign() * A.sq_norm().sqrt();
    
    let v = v.normalize();

    H - v.outer_dot(&v).scale(N::from_float(2.0)) 
}
