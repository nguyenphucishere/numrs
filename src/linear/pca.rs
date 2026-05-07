use crate::linear::svd::svd;
use crate::matrix::Matrix;
use crate::vector::Vector;
use crate::utils::numbers::Numeric;
use crate::linear::qr::qr;

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



pub fn randomized_pca_threshold<N: Numeric>(mat_data: &Matrix<N>, n_oversamples: usize,
    energy_threshold: N,
    random_data: Option<Matrix<N>>,
) -> (Matrix<N>, N) {

    todo!();
}