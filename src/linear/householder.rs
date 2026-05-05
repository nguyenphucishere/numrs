use crate::matrix::Matrix;
use crate::vector::Vector;
use crate::utils::numbers::Numeric;

pub fn householder<N: Numeric>(A: &Vector<N>) -> Matrix<N> {
    let H = Matrix::<N>::identity(A[..].len());
    let mut v = A.clone();

    v[0] = v[0] + A[0].sign() * A.sq_norm().sqrt();
    
    let v = v.normalize();

    H - v.outer_dot(&v).scale(N::from_float(2.0)) 
}

pub fn householder_reflection<N: Numeric>(A: &Matrix<N>) -> (Matrix<N>, Matrix<N>) {
    let (m, n) = A.shape();
    let mut H = Matrix::<N>::identity(m);
    let mut R = A.clone();

    for i in 0..n {
        if i + 1 >= m {
            break;
        }
        let x = &R.col(i)[i + 1..m];
        let h = householder(&Vector::from_arr(x));
        let h = Matrix::<N>::identity(m)
            .with_submatrix(i + 1, i + 1, &h);

        H = &H * &h;
        R = &h * &R;
    }

    (H, R)
}

pub fn hessenberg_reduction<N: Numeric>(A: &Matrix<N>) -> (Matrix<N>, Matrix<N>) {
    let (m, n) = A.shape();
    let mut Q: Matrix<N> = Matrix::<N>::identity(m);
    let mut R = A.clone();

    for i in 0..(n - 2) {

        let mut v = Vector::from_arr(&R.col(i)[i + 1..m]);
        v[0] = v[0] + v[0].sign() * v.sq_norm().sqrt();
        let v = &v.normalize()[..];

        // Calculate the row vector z^T = v^T * R
        // Length of z will be (n - i)
        let mut z_row = vec![N::zero(); n - i];
        for c in i..n {
            let mut dot_product = N::zero();
            // v has length m - (i + 1). R's corresponding rows start at i + 1.
            for r in (i + 1)..m {
                dot_product += v[r - (i + 1)] * R[r][c]; 
            }
            z_row[c - i] = dot_product;
        }

        // Update R directly in memory (R = R - 2 * v * z^T)
        for c in i..n {
            for r in (i + 1)..m {
                let update = N::from_float(2.0) * v[r - (i + 1)] * z_row[c - i];
                R[r][c] = R[r][c] + N::negative() * update;
            }
        }

        // Step 2A: Calculate the column vector z = R * v
        // Length of z will be m (Because we multiply ALL rows of R)
        let mut z_col = vec![N::zero(); m];
        for r in 0..m {
            let mut dot_product = N::zero();
            for c in (i + 1)..m {
                dot_product += R[r][c] * v[c - (i + 1)];
            }
            z_col[r] = dot_product;
        }

        // Step 2B: Update R directly in memory (R = R - 2 * z * v^T)
        for r in 0..m {
            for c in (i + 1)..m {
                let update = N::from_float(2.0) * z_col[r] * v[c - (i + 1)];
                R[r][c] = R[r][c] + N::negative() * update;
            }
        }

        // Calculate the column vector z_q = Q * v
        // Length of z_q is m
        let mut z_q = vec![N::zero(); m];
        for r in 0..m {
            let mut dot_product = N::zero();
            for c in (i + 1)..m {
                dot_product += Q[r][c] * v[c - (i + 1)];
            }
            z_q[r] = dot_product;
        }

        for r in 0..m {
            for c in (i + 1)..m {
                let update = N::from_float(2.0) * z_q[r] * v[c - (i + 1)];
                Q[r][c] = Q[r][c] + N::negative() * update;
            }
        }

    }
    (Q, R)
}