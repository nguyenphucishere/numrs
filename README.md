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
| `new` | `Matrix::new(rows, cols) -> Matrix<N>` | Create a zero-filled matrix. |
| `from_arr` | `Matrix::from_arr(&arr, rows, cols) -> Matrix<N>` | Build a matrix from a slice. |
| `from_vec` | `Matrix::from_vec(vec, rows, cols) -> Matrix<N>` | Build a matrix from an owned vector. |
| `from_space` | `Matrix::from_space(&space, as_col) -> Matrix<N>` | Convert a `Space` into a matrix representation. |
| `random` | `Matrix::random() -> Matrix<N>` | Placeholder for a random matrix constructor; currently `todo!()`. |
| `diag` | `Matrix::diag(&arr) -> Matrix<N>` | Create a diagonal matrix from values. |
| `is_diagonal` | `matrix.is_diagonal() -> bool` | Check whether all off-diagonal entries are zero. |
| `row_space` | `matrix.row_space() -> Space<N>` | Return the row space as a `Space`. |
| `col_space` | `matrix.col_space() -> Space<N>` | Return the column space as a `Space`. |
| `identity` | `Matrix::identity(n) -> Matrix<N>` | Create an identity matrix. |
| `shape` | `matrix.shape() -> (usize, usize)` | Return `(rows, cols)`. |
| `transpose` | `matrix.transpose() -> Matrix<N>` | Return the transpose. |
| `det` | `matrix.det() -> N` | Compute the determinant of a square matrix. |
| `augment` | `matrix.augment(&other) -> Matrix<N>` | Concatenate two matrices horizontally. |
| `dim_truncate` | `matrix.dim_truncate(rows, cols) -> ()` | Shrink a matrix in place to the given dimensions. |
| `inverse` | `matrix.inverse() -> Matrix<N>` | Compute the matrix inverse. |
| `safe_inverse` | `matrix.safe_inverse() -> Matrix<N>` | Compute the inverse with looser zero handling on diagonal matrices. |
| `scale` | `matrix.scale(factor) -> Matrix<N>` | Multiply all entries by a scalar and return the matrix. |
| `print` | `matrix.print() -> ()` | Print the matrix to stdout. |
| `print_round` | `matrix.print_round(decimals) -> ()` | Print the matrix with rounded formatting. |
| `trace` | `matrix.trace() -> N` | Compute the trace of a square matrix. |
| `pow` | `matrix.pow(exp) -> Matrix<N>` | Raise a square matrix to a positive integer power. |
| `forbenius_sq_norm` | `matrix.forbenius_sq_norm() -> N` | Compute the squared Frobenius norm. |
| `gauss_elim` | `matrix.gauss_elim() -> Matrix<N>` | Return the row-echelon form produced by Gaussian elimination. |
| `rank` | `matrix.rank() -> usize` | Compute the matrix rank. |
| `pseudo_inverse` | `matrix.pseudo_inverse() -> Matrix<N>` | Compute the Moore-Penrose-style pseudo-inverse used by the crate. |
| `main_diag` | `matrix.main_diag() -> Vec<N>` | Return the main diagonal as a vector. |
| `reshape` | `matrix.reshape(rows, cols) -> ()` | Change matrix shape in place without changing the underlying data count. |

## Vector<N>

`Vector<N>` is a thin wrapper around a column matrix and provides vector-oriented helpers.

| Name | Syntax | Purpose |
| --- | --- | --- |
| `new` | `Vector::new(dim) -> Vector<N>` | Create a zero vector of the given dimension. |
| `clone` | `vector.clone() -> Vector<N>` | Clone the vector. |
| `is_zero` | `vector.is_zero() -> bool` | Check whether all entries are zero. |
| `from_arr` | `Vector::from_arr(&arr) -> Vector<N>` | Build a vector from a slice. |
| `to_arr` | `vector.to_arr() -> Vec<N>` | Convert the vector into a `Vec<N>`. |
| `print` | `vector.print() -> ()` | Print the vector to stdout. |
| `sq_norm` | `vector.sq_norm() -> N` | Compute the squared Euclidean norm. |
| `normalize` | `vector.normalize() -> Vector<N>` | Return a unit-length version of the vector. |
| `dot` | `vector.dot(&other) -> N` | Compute the dot product with another vector. |
| `dot_vec` | `vector.dot_vec(&vec) -> N` | Compute the dot product with a raw `Vec<N>`. |
| `cross` | `vector.cross(&other) -> Vector<N>` | Compute the 3D cross product. |
| `cos_bwt` | `vector.cos_bwt(&other) -> N` | Compute the cosine of the angle between two vectors. |
| `outer_dot` | `vector.outer_dot(&other) -> Matrix<N>` | Compute the outer product as a matrix. |
| `proj_to` | `vector.proj_to(&u) -> Vector<N>` | Project the vector onto another vector. |

## Scalar<N>

`Scalar<N>` is a small wrapper for scalar values that helps keep matrix and vector scalar multiplication consistent.

| Name | Syntax | Purpose |
| --- | --- | --- |
| `new` | `Scalar::new(val) -> Scalar<N>` | Wrap a numeric value. |

## Space<N>

`Space<N>` stores a set of vectors and provides simple helpers for basis and dimension checks.

| Name | Syntax | Purpose |
| --- | --- | --- |
| `new` | `Space::new(vectors) -> Space<N>` | Create a space from a vector collection. |
| `to_matrix` | `space.to_matrix() -> Matrix<N>` | Convert the space into a matrix. |
| `is_basis` | `space.is_basis() -> bool` | Check whether the vectors form a basis. |
| `dim` | `space.dim() -> usize` | Compute the dimension of the span. |
| `len` | `space.len() -> usize` | Return the number of stored vectors. |

## Linear Module Functions

These functions are exposed under `numrs::linear`.

| Name | Syntax | Purpose |
| --- | --- | --- |
| `gramschmidt` | `linear::gramschmidt::gramschmidt(&space) -> Space<N>` | Orthonormalize a space using the Gram-Schmidt process. |
| `qr` | `linear::qr::qr(&matrix) -> (Matrix<N> as Q, Matrix<N> as R)` | Compute the QR decomposition. |
| `eig` | `linear::eig::eig(&matrix, iterations) -> (Matrix<N> as eigenvalues, Matrix<N> as eigenvectors)` | Compute eigenvalue and eigenvector. |
| `svd` | `linear::svd::svd(&matrix) -> (Matrix<N> as U, Matrix<N> as Sigma, Matrix<N> as V)` | Compute a singular value decomposition. |

### Linear module notes

- `linear::gramschmidt::gramschmidt(&space)` returns an orthonormal `Space`.
- `linear::pca` currently exists as a placeholder module and exposes no public functions yet.

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
- `linear::pca` is currently empty.

## Development

```bash
cargo build
cargo check
```

## License

Copyrights © 2026 NGUYEN VUONG THIEN PHUC.

This project is licensed under the Apache 2.0 license.