
use crate::matrix::Matrix;
use crate::utils::numbers::Numeric;
use crate::linear::qr::qr;
use crate::linear::householder::hessenberg_reduction;

fn extract_eigensystem<N: Numeric>(
    H: &Matrix<N>, 
    Q: &Matrix<N>
) -> (Matrix<N>, Matrix<N>) {
    let (m, n) = H.shape();
    assert_eq!(m, n, "Matrix must be square.");

    let eigenvalues = vec![N::zero(); n];
    
    let mut eigenvectors_H = Matrix::<N>::new(n, n); 

    // Solve (H - lambda * I)y = 0 for each eigenvalue.
    for i in 0..n {
        let lambda = eigenvalues[i];
        
        eigenvectors_H[i][i] = N::one(); 

        
        for j in (0..i).rev() {
            let mut sum = N::zero();
            
            
            for k in (j + 1)..=i {
                sum += H[j][k] * eigenvectors_H[k][i];
            }
            
            let diagonal_diff = H[j][j] + N::negative() * lambda;
            
            // Mathematical safety: Prevent division by zero if eigenvalues are identical
            let denom = if diagonal_diff.is_zero() {
                
                if !diagonal_diff.is_negative() { N::eps() } else { N::eps() * N::negative() }
            } else {
                diagonal_diff
            };
            eigenvectors_H[j][i] = N::negative() * sum / denom;
        }
    }

    // Original Eigenvectors X = Q * Y
    let mut final_eigenvectors = Matrix::<N>::new(n, n);
    
    for col in 0..n {
        for row in 0..n {
            let mut dot_product = N::zero();
            
            /* THE HARDWARE CHEAT CODE:
                Because eigenvectors_H (Y) was built via backward substitution, 
                all elements below the diagonal (k > col) are perfectly zero.

                Only loop from k = 0 to k = col. 
            */
            for k in 0..=col { 
                dot_product += Q[row][k] * eigenvectors_H[k][col];
            }
            
            final_eigenvectors[row][col] = dot_product;
        }

        let mut sq_norm = N::zero();
        for row in 0..n {
            sq_norm += final_eigenvectors[row][col] * final_eigenvectors[row][col];
        }
        let norm = sq_norm.sqrt();
        
        if norm > N::zero() {
            for row in 0..n {
                final_eigenvectors[row][col] = final_eigenvectors[row][col] / norm;
            }
        }
    }

    (Matrix::diag(&H.main_diag()), final_eigenvectors)
}

pub fn eig<N: Numeric>(A: &Matrix<N>, iterations: Option<usize>) -> (Matrix<N>, Matrix<N>) {
    let (m, n) = A.shape();

    if m != n {
        panic!("Matrix must be square for eigenvalue decomposition.");
    }

    /* 
        Method 1 -- fast, compact, easy to implement, but less accurate for repeated eigenvalues.

        Sometimes it can even fail to converge for certain matrices, 
        especially those with closely spaced or repeated eigenvalues.
    */
        // let mut eigenvectors = Matrix::identity(n);
        // let mut eigenvalues = A.clone();

        // for _ in 0..iterations.unwrap_or(1000) {

        //     let (Q, R) = qr(&eigenvalues);
        //     eigenvalues = &R * &Q;
        //     eigenvectors = &eigenvectors * &Q;
        // }

    // End Method 1

    /* Method 2 -- more accurate for repeated eigenvalues, more complex to implement.

        This method is based on the QR algorithm with shifts, 
        which is more robust and can handle repeated eigenvalues better.

        However, sometimes matrix shifts cannot be chosen well, 
        leading to slow convergence or even divergence in some cases.
    */
    
        let mut eigenvectors = Matrix::identity(n);
        let mut eigenvalues = A.clone();
        for _ in 0..iterations.unwrap_or(10000) {
            
            let shift = eigenvalues[n-1][n-1];
            let shifted_matrix = &eigenvalues - Matrix::identity(n).scale(shift);
            
            let (Q, R) = qr(&shifted_matrix);
            
            eigenvalues = &R * &Q + Matrix::identity(n).scale(shift);
            eigenvectors = &eigenvectors * &Q;

            //check for convergence
            if (0..n-1).all(|i| eigenvalues[i+1][i].is_zero()) {
                break;
            }
        }
    
    // End Method 2

    /* Method 3 -- heissenberg reduction + QR algorithm, 
        more accurate for repeated eigenvalues, but more complex to implement and slower than method 2.
    */
        // let max_iterations = iterations.unwrap_or(1000);
        // let mut iter_count = 0;
        // let (mut Q, mut H) = hessenberg_reduction(A);


        // let (_, n) = H.shape();
        // let mut active_n = n;

            
        // while active_n > 1 {
    
        //     // Check the bottom-most subdiagonal element.
        //     // If it's effectively zero, the eigenvalue is isolated
    
        //     if active_n > 1 && H[active_n - 1][active_n - 2].is_zero() {
                
        //         H[active_n - 1][active_n - 2] = N::zero();
                
        //         active_n -= 1;
        //         iter_count = 0;
        //         continue;
        //     }

        //     if iter_count > max_iterations {
        //         panic!("QR Algorithm failed to converge");
        //     }


        //     // Combined with method 2
        //     let mu = H[active_n - 1][active_n - 1];
        //     for i in 0..active_n {
        //         H[i][i] = H[i][i] + N::negative() * mu;
        //     }

        //     // Store the rotations (c, s) to apply them on the right side later
        //     // Length only needs to be active_n - 1, as we do 2x2 rotations
        //     let mut rotations: Vec<(N, N)> = Vec::with_capacity(active_n - 1);

        //     for k in 0..(active_n - 1) {
        //         let a = H[k][k];
        //         let b = H[k + 1][k];

        //         let r = (a * a + b * b).sqrt();

        //         let (c, s) = if !r.is_zero() {
        //             (a / r, b / r)
        //         } else {
        //             (N::one(), N::zero())
        //         };
        //         rotations.push((c, s));

        //         /* NOTE: We only need to update from column k to the end, 
        //         because everything to the left is already zero */

        //         for col in k..active_n {
        //             let top_val = H[k][col];
        //             let bot_val = H[k + 1][col];

        //             H[k][col] = c * top_val + s * bot_val;
        //             H[k + 1][col] = N::negative() * s * top_val + c * bot_val;
        //         }
        //     }


        //     // Apply the rotations to the columns: H = R * Q
        //     for k in 0..(active_n - 1) {
        //         let (c, s) = rotations[k];

        //         for row in 0..=k + 1 {
        //             let left_val = H[row][k];
        //             let right_val = H[row][k + 1];

        //             H[row][k] = c * left_val + s * right_val;
        //             H[row][k + 1] = N::negative() * s * left_val + c * right_val;
        //         }

        //         for row in 0..m {
        //             let left_val = Q[row][k];
        //             let right_val = Q[row][k + 1];
        
        //             Q[row][k] = c * left_val + s * right_val;
        //             Q[row][k + 1] = N::negative() * s * left_val + c * right_val;
        //         }

        //     }


        //     // restore the shift
        //     for i in 0..active_n {
        //         H[i][i] += mu;
        //     }

        //     iter_count += 1;

        // }

        // /* 
        //     TODO: Shorten the syntax for extracting eigenvalues and eigenvectors from the final H and Q matrices.
        //     I feel like there should be a more elegant way to do this, 
        //     but for now I'll just write it out verbosely.
        // */

        // let (eigenvalues, eigenvectors) = extract_eigensystem(&H, &Q);

    //End method 3
    
    (eigenvalues, eigenvectors)
    
}