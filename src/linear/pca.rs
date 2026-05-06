use crate::linear::svd::svd;
use crate::matrix::Matrix;
use crate::vector::Vector;
use crate::utils::numbers::Numeric;
use crate::linear::qr::qr;

pub fn pca<N: Numeric>(data: Matrix<N>, n_components: usize) -> Matrix<N> {

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

    // Project the data onto the selected eigenvectors
    centered_data * selected_eigenvectors

}

pub fn randomized_pca<N: Numeric>(mat_data: &Matrix<N>, n_components: usize, n_oversamples: usize,
    random_data: Option<Matrix<N>>

) -> Matrix<N> {

    // let random_projection = Matrix::random(mat_data.shape().1, n_components, rand::random(), Some(n_oversamples));

    let random_projection = if let Some(data) = random_data {
        data
    } else {
        Matrix::random(mat_data.shape().1, n_components, rand::random(), Some(n_oversamples))
    };

    let projected_data = mat_data * &random_projection;

    let (Q, _) = qr(&projected_data);
    
    let projected_data = Q.transpose() * mat_data;
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

    mat_data * selected_eigenvectors
}