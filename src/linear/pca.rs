use crate::linear::svd::svd;
use crate::matrix::Matrix;
use crate::vector::Vector;
use crate::utils::numbers::Numeric;
use crate::linear::qr::qr;
use crate::space::Space;

pub fn pca<N: Numeric>(data: Matrix<N>, n_components: usize) -> (Matrix<N>, N) {

    let centered_data = data.center_matrix_axis(0);

    // Method 1: Compute the covariance matrix

        // let covariance_matrix = 
        // (centered_data.transpose() * &centered_data)
        // .scale(
        //     N::from_float(1.0 / (centered_data.shape().0 as f64 - 1.0))
        // );

        // let (eigenvalues, eigenvectors) = eig(&covariance_matrix, None);

    // End Method 1

    // Method 2: Directly compute eigenvalues and eigenvectors from the centered data

    /* By this way, the eigenvalues will be scaled by the number of samples, 
    but since we only care about the relative magnitudes for PCA, 
    this won't affect the selection of principal components. */

        // let (eigenvalues, eigenvectors) = 
        //     eig(&(centered_data.transpose() * &centered_data), None);

    // End Method 2

    // Method 3: Using SVD to compute eigenvalues and eigenvectors

    /* The v vectors from SVD of the centered data are the eigenvectors of the covariance matrix
    as v is the eigenvectors of centered_data.transpose() * centered_data, 
    which is the covariance matrix (up to scaling) */

        let (_, eigenvalues, v) = svd(&centered_data);
        
        let eigenvectors = v.transpose();

    // End Method 3
    
    // Sort eigenvalues and corresponding eigenvectors in descending order
    let mut eigen_pairs: Vec<(N, Vector<N>)> = eigenvalues.main_diag().into_iter().zip(eigenvectors.col_space().vectors).collect();
    eigen_pairs.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    // Select the top n_components eigenvectors
    let selected_eigenvectors: Matrix<N> = Matrix::from_columns(
        &eigen_pairs.iter().take(n_components)
        .map(|pair| pair.1.clone())
        .collect::<Vec<Vector<N>>>()
    );

    let final_mat = &centered_data * &selected_eigenvectors;
    let approximation_error = (
        centered_data.forbenius_norm() * centered_data.forbenius_norm() + N::negative() * 
        final_mat.forbenius_norm() * final_mat.forbenius_norm()
    ).sqrt();

    // Project the data onto the selected eigenvectors
    (final_mat, approximation_error)
}

pub fn randomized_pca<N: Numeric>(mat_data: &Matrix<N>, n_components: usize, n_oversamples: usize,
    random_data: Option<Matrix<N>>
) -> (Matrix<N>, N) {

    let centered_data = mat_data.center_matrix_axis(0);
    let random_projection = if let Some(data) = random_data {
        data
    } else {
        Matrix::random(mat_data.shape().1, n_components, rand::random(), Some(n_oversamples))
    };

    let projected_data = &centered_data * &random_projection;

    let (Q, _) = qr(&projected_data);
    
    let projected_data = Q.transpose() * &centered_data;
    let (_, eigenvalues, v) = svd(&projected_data);

    let eigenvectors = v.transpose();
    
    let mut eigen_pairs: Vec<(N, Vector<N>)> = eigenvalues.main_diag().into_iter().zip(eigenvectors.col_space().vectors).collect();
    eigen_pairs.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    // Select the top n_components eigenvectors
    let selected_eigenvectors: Matrix<N> = Matrix::from_columns(
        &eigen_pairs.iter().take(n_components)
        .map(|pair| pair.1.clone())
        .collect::<Vec<Vector<N>>>()
    );

    let final_mat = &centered_data * &selected_eigenvectors;

    // Method 1: reconstruct then calculate the approximation error
    // let approximation_error = 
    //     (centered_data - &final_mat * selected_eigenvectors.transpose()).forbenius_norm();

    // Method 2: using Pyhtagorean theorem
    let approximation_error = (
        centered_data.forbenius_norm() * centered_data.forbenius_norm() + N::negative() * 
        final_mat.forbenius_norm() * final_mat.forbenius_norm()
    ).sqrt();

    (final_mat, approximation_error)
}



/*  From the paper "Finding structure with randomness: 
        Stochastic algorithms for constructing approximate matrix decompositions" 
    by Halko, Martinsson, and Tropp (2011)
*/
pub fn randomized_pca_threshold<N: Numeric>(mat_data: &Matrix<N>, n_oversamples: usize,
    energy_threshold: N
) -> (Matrix<N>, N) {

    let mut Q = Space::empty();
    let centered_data = mat_data.center_matrix_axis(0);
    let mut current_error = estimate_intial_error(&centered_data, None);
    let tolerance = current_error * (N::one() + N::negative() * energy_threshold);
    
    while current_error > tolerance {
        
        let seed = rand::random();
        let omega_block = Vector::random(centered_data.shape().1, seed, Some(n_oversamples));
        

        let y_block = &centered_data * omega_block;
        
        Q.append(y_block);
        Q.orthogonize();

        
        current_error = estimate_subspace_error(&centered_data, &Q.to_matrix(), None);
    }

    let B = Q.to_matrix().transpose() * &centered_data;
    let (_, _, eigenvectors) = svd(&B);

    (centered_data * eigenvectors, current_error)
}

fn estimate_intial_error<N: Numeric>(mat_data: &Matrix<N>, num_probes: Option<usize>) -> N {
    // repeat myself for performance.
    
    let (_, n) = mat_data.shape();
    let num_probes = num_probes.unwrap_or(10);
    let seed = rand::random();
    let mut max_error = N::from_float(0.0);
    
    for _ in 0..num_probes {
        let omega = Matrix::random(n, 1, seed, None);
        let current_error_sq = (mat_data * &omega).forbenius_norm();

        if max_error < current_error_sq {
            max_error = current_error_sq;
        }
    }

    max_error.sqrt()
}

fn estimate_subspace_error<N: Numeric>(mat_data: &Matrix<N>, orth_basis: &Matrix<N>,
    num_probes: Option<usize>
) -> N {
    let (_, n) = mat_data.shape();
    let num_probes = num_probes.unwrap_or(10);
    let seed = rand::random();
    let mut max_error = N::from_float(0.0);
    
    for _ in 0..num_probes {
        let omega = Matrix::random(n, 1, seed, None);

        let mat_x_omega = mat_data * &omega;
        
        // captured_vector = orth_basis * (orthbasis.transpose * mat_x_omega)
        let captured_vector = orth_basis * (orth_basis.transpose() * &mat_x_omega);

        // residual_vector = (A - QQ^T A) * omega = mat_x_omega - QQ^T * omega
        let residual_vector = &mat_x_omega - &captured_vector;
        
        let current_error_sq = residual_vector.forbenius_norm();
        

        if max_error < current_error_sq {
            max_error = current_error_sq;
        }
    }

    max_error.sqrt()
}