# numrs

`numrs` is a Rust linear algebra library with a NumPy-like feel for dense matrices, vectors, scalar wrappers, and basic vector-space operations. The crate is intentionally small and direct, with APIs designed to stay close to the underlying mathematics.

This library originated as a personal project to support my major assignment research on randomized PCA (Linear Algebra - HCMUT), and to create a simple, self-contained implementation of common linear algebra operations without external dependencies.

`numrs` is made for educational purposes and is not optimized for performance. It is intended to be a learning tool for understanding linear algebra algorithms and Rust programming, rather than a production-ready and high-performance library.

## Installation

```toml
[dependencies]
numrs = { path = "./numrs" }
```
This assumes you have the `numrs` crate in a local directory. Adjust the path as needed for your project structure.

## Public Modules

- `matrix` for dense matrix operations.
- `vector` for vector-specific operations.
- `scalar` for scalar wrappers.
- `space` for spans, bases, and dimension helpers.
- `linear` for decomposition algorithms.
- `utils` for numeric traits and shared helpers.

## Matrix<N>

`Matrix<N>` is the core type in the crate. It stores dense matrix data and supports construction, inspection, transformations, and linear algebra operations.

| Name | Syntax | Purpose |
| --- | --- | --- |
| `new` | `Matrix::new(rows, cols) -> Matrix<N>` | Create a zero matrix with the requested dimensions. |
| `slice_matrix` | `matrix.slice_matrix(row_range, col_range) -> Matrix<N>` | Return a copied submatrix from the selected row and column ranges. |
| `cols` | `matrix.cols(range) -> Matrix<N>` | Build a matrix from a selected range of columns. |
| `from_arr` | `Matrix::from_arr(&arr, rows, cols) -> Matrix<N>` | Build a matrix from a flat slice in row-major order. |
| `from_vec` | `Matrix::from_vec(&vec, rows, cols) -> Matrix<N>` | Build a matrix by cloning a flat buffer. |
| `from_space` | `Matrix::from_space(&space, as_col) -> Matrix<N>` | Convert a `Space` into a matrix with vectors as rows or columns. |
| `from_rows` | `Matrix::from_rows(&rows) -> Matrix<N>` | Build a matrix from a slice of row vectors. |
| `from_columns` | `Matrix::from_columns(&columns) -> Matrix<N>` | Build a matrix from a slice of column vectors. |
| `random` | `Matrix::random() -> Matrix<N>` | Placeholder constructor for a random matrix; currently `todo!()`. |
| `diag` | `Matrix::diag(&arr) -> Matrix<N>` | Create a diagonal matrix from the supplied values. |
| `is_diagonal` | `matrix.is_diagonal() -> bool` | Return `true` when every off-diagonal entry is zero. |
| `row_space` | `matrix.row_space() -> Space<N>` | Return the span of the matrix rows as a `Space`. |
| `col_space` | `matrix.col_space() -> Space<N>` | Return the span of the matrix columns as a `Space`. |
| `col` | `matrix.col(index) -> Vector<N>` | Return a matrix column as a vector. |
| `with_submatrix` | `matrix.with_submatrix(row_start, col_start, &submatrix) -> Matrix<N>` | Return a copy with a submatrix inserted at the given position. |
| `identity` | `Matrix::identity(n) -> Matrix<N>` | Create an $n \times n$ identity matrix. |
| `shape` | `matrix.shape() -> (usize, usize)` | Return the matrix dimensions as `(rows, cols)`. |
| `transpose` | `matrix.transpose() -> Matrix<N>` | Return a transposed copy of the matrix. |
| `det` | `matrix.det() -> N` | Compute the determinant of a square matrix. |
| `augment` | `matrix.augment(&other) -> Matrix<N>` | Append another matrix to the right of this one. |
| `dim_truncate` | `matrix.dim_truncate(rows, cols) -> ()` | Truncate the matrix in place to the requested shape. |
| `inverse` | `matrix.inverse() -> Matrix<N>` | Compute the matrix inverse using the crate's standard routine. |
| `safe_inverse` | `matrix.safe_inverse() -> Matrix<N>` | Compute an inverse with relaxed zero handling for diagonal matrices. |
| `scale` | `matrix.scale(factor) -> Matrix<N>` | Multiply every entry by a scalar factor. |
| `print` | `matrix.print() -> ()` | Print the matrix in its default formatting. |
| `print_round` | `matrix.print_round(decimals) -> ()` | Print the matrix with rounded decimal formatting. |
| `trace` | `matrix.trace() -> N` | Compute the sum of the diagonal entries. |
| `pow` | `matrix.pow(exp) -> Matrix<N>` | Raise a square matrix to an integer power (`exp >= 1`). |
| `forbenius_sq_norm` | `matrix.forbenius_sq_norm() -> N` | Compute the squared Frobenius norm of the matrix. |
| `forbenius_norm` | `matrix.forbenius_norm() -> N` | Compute the Frobenius norm of the matrix. |
| `mean` | `matrix.mean() -> N` | Compute the average of all matrix entries. |
| `center_matrix` | `matrix.center_matrix() -> Matrix<N>` | Return a copy with the global mean subtracted from every entry. |
| `mean_axis` | `matrix.mean_axis(axis) -> Vector<N>` | Compute means by axis (`0`: column means, `1`: row means). |
| `center_matrix_axis` | `matrix.center_matrix_axis(axis) -> Matrix<N>` | Return a copy centered by axis (`0`: per-column, `1`: per-row). |
| `gauss_elim` | `matrix.gauss_elim() -> Matrix<N>` | Return the row-echelon form from Gaussian elimination. |
| `rank` | `matrix.rank() -> usize` | Compute the rank from the row-echelon form. |
| `pseudo_inverse` | `matrix.pseudo_inverse() -> Matrix<N>` | Compute the pseudo-inverse used by the crate. |
| `main_diag` | `matrix.main_diag() -> Vec<N>` | Return the main diagonal as a `Vec`. |
| `reshape` | `matrix.reshape(rows, cols) -> ()` | Change the matrix shape in place without changing the element count. |

## Vector<N>

`Vector<N>` is a thin wrapper around a column matrix and provides vector-oriented helpers.

| Name | Syntax | Purpose |
| --- | --- | --- |
| `new` | `Vector::new(dim) -> Vector<N>` | Create a zero vector with the requested dimension. |
| `from_matrix` | `Vector::from_matrix(matrix) -> Vector<N>` | Wrap an `n x 1` matrix as a vector. |
| `from_vec` | `Vector::from_vec(vec) -> Vector<N>` | Build a column vector from an owned buffer. |
| `standard_basis` | `Vector::standard_basis(dim, index) -> Vector<N>` | Create a standard basis vector with a 1 at the selected index. |
| `zero_at` | `Vector::zero_at(dim, zero_index) -> Vector<N>` | Create a vector filled with ones except for a zero at the selected index. |
| `clone` | `vector.clone() -> Vector<N>` | Return a deep copy of the vector. |
| `is_zero` | `vector.is_zero() -> bool` | Return `true` when all entries are zero. |
| `from_arr` | `Vector::from_arr(&arr) -> Vector<N>` | Build a vector from a flat slice. |
| `to_arr` | `vector.to_arr() -> Vec<N>` | Convert the vector into a flat `Vec`. |
| `print` | `vector.print() -> ()` | Print the vector in its default formatting. |
| `sq_norm` | `vector.sq_norm() -> N` | Compute the squared Euclidean norm. |
| `normalize` | `vector.normalize() -> Vector<N>` | Return a unit vector in the same direction. |
| `dot` | `vector.dot(&other) -> N` | Compute the dot product with another vector. |
| `dot_vec` | `vector.dot_vec(&vec) -> N` | Compute the dot product with a raw vector buffer. |
| `cross` | `vector.cross(&other) -> Vector<N>` | Compute the 3D cross product. |
| `cos_bwt` | `vector.cos_bwt(&other) -> N` | Compute the cosine of the angle between two vectors. |
| `outer_dot` | `vector.outer_dot(&other) -> Matrix<N>` | Compute the outer product as a matrix. |
| `transpose` | `vector.transpose() -> Matrix<N>` | Return the transpose as a `1 x n` matrix. |
| `proj_to` | `vector.proj_to(&u) -> Vector<N>` | Project the vector onto another vector. |

## Scalar<N>

`Scalar<N>` is a small wrapper for scalar values that helps keep matrix and vector scalar multiplication consistent.

| Name | Syntax | Purpose |
| --- | --- | --- |
| `new` | `Scalar::new(val) -> Scalar<N>` | Wrap a numeric value for scalar-based operations. |

## Space<N>

`Space<N>` stores a set of vectors and provides simple helpers for basis and dimension checks.

| Name | Syntax | Purpose |
| --- | --- | --- |
| `new` | `Space::new(vectors) -> Space<N>` | Create a space from a collection of vectors. |
| `to_matrix` | `space.to_matrix() -> Matrix<N>` | Convert the stored vectors into a matrix. |
| `is_basis` | `space.is_basis() -> bool` | Return `true` when the vectors form an invertible square matrix. |
| `dim` | `space.dim() -> usize` | Compute the dimension of the span. |
| `len` | `space.len() -> usize` | Return how many vectors are stored. |

## Linear Module Functions

These functions are exposed under `numrs::linear`.

| Name | Syntax | Purpose |
| --- | --- | --- |
| `gramschmidt` | `linear::gramschmidt::gramschmidt(&space) -> Space<N>` | Orthonormalize a space with the Gram-Schmidt process. |
| `qr` | `linear::qr::qr(&matrix) -> (Matrix<N> as Q, Matrix<N> as R)` | Compute the QR decomposition of a matrix. |
| `eig` | `linear::eig::eig(&matrix, iterations) -> (Matrix<N> as eigenvalues, Matrix<N> as eigenvectors)` | Compute approximate eigenvalues and eigenvectors. |
| `svd` | `linear::svd::svd(&matrix) -> (Matrix<N> as U, Matrix<N> as Sigma, Matrix<N> as V)` | Compute the singular value decomposition. |
| `householder` | `linear::householder::householder(&vector) -> Matrix<N>` | Build a Householder reflection matrix from a vector. |
| `pca` | `linear::pca::pca(data, n_components) -> Matrix<N>` | Project centered data onto the top principal components. |
| `test_svd` | `linear::svd::test_svd(&matrix) -> ()` | Print the intermediate matrices used by the SVD test routine. |

### Linear module notes

- `linear::gramschmidt::gramschmidt(&space)` returns an orthonormal `Space`.
- `linear::pca::pca(data, n_components)` returns the data projected onto the top principal components.

## Trait & Operator Summary (by type)

**__IMPORTANT NOTE:__**
- The operator tables below are not exhaustive. They cover the most common and intuitive operations, but many more trait implementations exist for various owned/borrowed combinations.
- Passing ownership (not using `&`) can be use for in-place operations where supported, but borrowing is more common for arithmetic ops.


### Matrix<N>

| Trait / Op | Example syntax | Effect |
| --- | --- | --- |
| `Clone` | `let m2 = m.clone()` | Deep copy of matrix data. |
| `Iterator` | `m.next()` | Pop and return first element from backing buffer. |
| `FromIterator` | `iter.collect::<Matrix<_>>()` | Build single-column matrix from iterator. |
| `Index<usize>` | `m[row]` | Returns row slice (`[N]`). |
| `IndexMut<usize>` | `m[row][col] = val` | Mutable element access. |
| `Index<Range>` / `Index<RangeFull>` | `m[start..end]`, `m[..]` | Row-range or flattened slice access. |
| `Add/Sub/Mul` (many impls) | `&a + &b`, `a + &b`, `&a * &b` | Matrix arithmetic for owned/borrowed combos. |
| `AddAssign` | `m += &other` or `m += other` | In-place addition for owned/borrowed RHS. |

### Vector<N>

| Trait / Op | Example syntax | Effect |
| --- | --- | --- |
| `Clone` | `let v2 = v.clone()` | Deep copy. |
| `Index` / `IndexMut` | `v[i]`, `v[i] = x` | Element access and mutation. |
| Element-wise `Mul` | `&v1 * &v2`, `v1 * v2` | Element-wise multiplication. |
| Scalar `Mul` (raw `N`) | `&v * 2.0` or `v * 2.0` | Scale vector by numeric value. |
| Scalar `Mul` (via `Scalar`) | `&Scalar::new(2.0) * &v` | Scale using `Scalar` wrapper. |
| Vector/Matrix multiply | `&matrix * &v`, `&v * &matrix` | Matrix-vector and vector-matrix multiplication. |
| `Add` / `AddAssign` | `&v1 + &v2`, `v += &other` | Vector addition (owned/borrowed combos). |

### Scalar<N>

| Trait / Op | Example syntax | Effect |
| --- | --- | --- |
| `Index<RangeFull>` | `scalar[..]` | Access wrapped numeric value by full-range indexing. |
| `Mul<Matrix>` variants | `&scalar * &matrix`, `scalar * matrix`, `matrix * scalar` | Scale matrices by scalar (owned/borrowed combos). |

## Example

```rust
use numrs::linear;
use numrs::matrix::Matrix;
use numrs::vector::Vector;

fn main() {
    let a = Matrix::from_arr(&[1.0, 2.0, 3.0, 4.0], 2, 2);
    let b = Matrix::identity(2);
    let v = Vector::from_arr(&[1.0, 2.0]);

    let c = &a + &b;
    let d = &a * &b;
    let projection = v.proj_to(&Vector::from_arr(&[2.0, 0.0]));
    let (u, sigma, v_t) = linear::svd::svd(&a);

    println!("shape: {:?}", a.shape());
    println!("determinant: {}", a.det());
    println!("projection: {:?}", projection.to_arr());
    println!("U shape: {:?}", u.shape());
    println!("Sigma shape: {:?}", sigma.shape());
    println!("V shape: {:?}", v_t.shape());

    c.print();
    d.print();
}
```

## Numeric Support

The public types are generic over `Numeric`. The trait is implemented for the standard signed and unsigned integer types, plus `f32` and `f64`.

For numerical routines such as inversion, normalization, and decomposition, floating-point types are usually the best fit because the implementation uses tolerance-based comparisons.

## Notes

- Many methods validate dimensions and will panic on invalid input.
- `Matrix::random()` is currently unimplemented.
- The crate is not optimized for performance and is intended for educational use. For large datasets or production use, consider a more mature library.

## Development logs

#### 2025-05-05
```rust
    A = [
        [12.0,  5.0,  2.0],
        [ 8.0,  5.0,  2.0],
        [10.0,  6.0,  2.0],
        [10.0,  4.0,  2.0],
        [10.0,  5.0,  2.0],
    ]
```
This is the test data for PCA, which is just newly implemented. The test works fine, my PCA use traditional SVD-based approach, which is not the most efficient but is straightforward and works well for small datasets. But when I ran SVD on this array, the results were not as expected. The squared value of $\Sigma$ should be the eigenvalues of $A^T A$, but they were not - they are negative (?!). 
Turns out, the QR alogirthm (used when fiding eigenvalues/eigenvectors for $A^T A$) is not converge for this matrix. The only reason for PCA test to pass but QR algorithm to fail is that the PCA find the eigenvalues of $A^T A$ **after centered**, but SVD find the eigenvalues of raw $A^T A$. I try to run QR algorithm with shifts, but the result did not change. I don't know why the centering step makes the QR algorithm converge, I think if I find the relationship between the eigenvalues of $A^T A$ and the eigenvalues of $(A - mean)^T (A - mean)$, I might be able to find a workaround for this issue. 
But for now, I have implement the Heissenberg reduction, which is a more stable method for finding eigenvalues/eigenvectors.

## Development

```bash
cargo build
cargo check
```

## License

Copyrights © 2026 NGUYEN VUONG THIEN PHUC.

This project is licensed under the Apache 2.0 license.