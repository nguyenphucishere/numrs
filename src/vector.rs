use crate::matrix::Matrix;
use crate::utils::numbers::Numeric;
use crate::scalar::Scalar;
use std::ops::{Index, Range, RangeFull, IndexMut, Mul, Add, AddAssign};

pub struct Vector<N>{
    data: Matrix<N>
}

impl<N: Numeric> Vector<N>{
    pub fn new(dim: usize) -> Self{
        Vector { data: Matrix::<N>::new(dim, 1) }
    }

    pub fn clone(&self) -> Self{
        Vector { data: self.data.clone() }
    }

    pub fn is_zero(&self) -> bool{
        self.data[..].iter().all(|&x| x.is_zero())
    }

    pub fn from_arr(arr: &[N]) -> Self{
        Vector { data: Matrix::from_arr(arr, arr.len(), 1) }
    }

    pub fn to_arr(&self) -> Vec<N>{
        self.data[..].to_vec()
    }

    pub fn print(&self){
        self.data.print();
    }

    pub fn sq_norm(&self) -> N{
        self.data[..].iter().map(|&x| x * x).sum::<N>()
    }

    pub fn normalize(&self) -> Self{
        let norm = self.sq_norm().sqrt();
        if norm.is_zero() {
            return Vector::new(self.data.shape().0); // Return a zero vector if the norm is zero
        }

        Vector { data: &self.data * &Scalar::new(N::one() / norm) }
    }

    pub fn dot(&self, other: &Self) -> N{
        if self.data.shape() != other.data.shape(){
            panic!("Dimension mismatch for vector dot product!");
        }

        self.data[..].iter().zip(other.data[..].iter()).map(|(&x, &y)| x * y).sum::<N>()
    }

    pub fn dot_vec(&self, other: &Vec<N>) -> N{
        if self.data.shape() != (other.len(), 1){
            panic!("Dimension mismatch for vector dot product!");
        }

        self.data[..].iter().zip(other.iter()).map(|(&x, &y)| x * y).sum::<N>()
    }

    pub fn cross(&self, other: &Self) -> Self{
        if self.data.shape() != (3, 1) || other.data.shape() != (3, 1){
            panic!("Cross product is only defined for 3D vectors!");
        }

        let x1 = self[0];
        let y1 = self[1];
        let z1 = self[2];

        let x2 = other[0];
        let y2 = other[1];
        let z2 = other[2];

        Vector::from_arr(&[
            y1 * z2 + N::negative() * z1 * y2,
            z1 * x2 + N::negative() * x1 * z2,
            x1 * y2 + N::negative() * y1 * x2
        ])
    }

    pub fn cos_bwt(&self, other: &Self) -> N{
        let dot_product = self.dot(other);
        let norms_product = (self.sq_norm() * other.sq_norm()).sqrt();

        if norms_product.is_zero(){
            panic!("Cannot compute angle with a zero vector!");
        }

        dot_product / norms_product
    }

    pub fn outer_dot(&self, other: &Self) -> Matrix<N>{
        if self.data.shape() != other.data.shape(){
            panic!("Dimension mismatch for outer dot product!");
        }

        Matrix::<N>::from_vec(
            self.data[..].iter().flat_map(|&x| other.data[..].iter().map(move |&y| x * y)).collect(),
            self.data.shape().0,
            other.data.shape().0,
        )
    }

    pub fn proj_to(&self, u: &Self) -> Vector<N>{
        if self.data.shape() != u.data.shape(){
            panic!("Dimension mismatch for vector projection!");
        }

        let dot_product = self.dot(u);
        let norm_sq = u.sq_norm();

        if norm_sq.is_zero(){
            panic!("Cannot project onto a zero vector!");
        }

        u * (dot_product / norm_sq)
}

}

impl<N: Numeric> Clone for Vector<N>{
    fn clone(&self) -> Self{
        Vector { data: self.data.clone() }
    }
}

impl<N: Numeric> Index<usize> for Vector<N>{
    type Output = N;

    fn index(&self, index: usize) -> &N{
        &self.data[index][0]
    }
}

impl<N: Numeric> IndexMut<usize> for Vector<N>{
    fn index_mut(&mut self, index: usize) -> &mut N{
        &mut self.data[index][0]
    }
}

impl<N: Numeric> Index<RangeFull> for Vector<N>{
    type Output = [N];

    fn index(&self, _index: RangeFull) -> &[N]{
        &self.data[..]
    }
}

impl<N: Numeric> Index<Range<usize>> for Vector<N>{
    type Output = [N];

    fn index(&self, index: Range<usize>) -> &[N]{
        &self.data[index]
    }
}


// vector mul

impl<N: Numeric> Mul for &Vector<N>{
    type Output = Vector<N>;

    fn mul(self, other: Self) -> Vector<N>{
        let dim = self.data.shape().0;
        let mut result = Vector::<N>::new(dim);

        for i in 0..dim{
            result[i] = (self[i] * other[i]).ground_if_zero();
        }

        result
    }
}

impl<N: Numeric> Mul for Vector<N>{
    type Output = Vector<N>;

    fn mul(self, other: Self) -> Vector<N>{
        let dim = self.data.shape().0;
        let mut result = Vector::<N>::new(dim);

        for i in 0..dim{
            result[i] = (self[i] * other[i]).ground_if_zero();
        }

        result
    }
}

impl<N: Numeric> Mul<Vector<N>> for &Vector<N>{
    type Output = Vector<N>;

    fn mul(self, other: Vector<N>) -> Vector<N>{
        let dim = self.data.shape().0;
        let mut result = Vector::<N>::new(dim);

        for i in 0..dim{
            result[i] = (self[i] * other[i]).ground_if_zero();
        }

        result
    }
}

impl<N: Numeric> Mul<&Vector<N>> for Vector<N>{
    type Output = Vector<N>;

    fn mul(self, other: &Vector<N>) -> Vector<N>{
        let dim = self.data.shape().0;
        let mut result = Vector::<N>::new(dim);

        for i in 0..dim{
            result[i] = (self[i] * other[i]).ground_if_zero();
        }

        result
    }
}


// scalar vs vector multiplication
impl<N: Numeric> Mul<&Scalar<N>> for &Vector<N>{
    type Output = Vector<N>;

    fn mul(self, other: &Scalar<N>) -> Vector<N>{
        let dim = self.data.shape().0;
        let mut result = Vector::<N>::new(dim);

        for i in 0..dim{
            result[i] = (self[i] * other[..]).ground_if_zero();
        }

        result
    }
}

impl<N: Numeric> Mul<&Scalar<N>> for Vector<N>{
    type Output = Vector<N>;

    fn mul(self, other: &Scalar<N>) -> Vector<N>{
        let dim = self.data.shape().0;
        let mut result = Vector::<N>::new(dim);

        for i in 0..dim{
            result[i] = (self[i] * other[..]).ground_if_zero();
        }

        result
    }
}

impl<N: Numeric> Mul<Scalar<N>> for Vector<N>{
    type Output = Vector<N>;

    fn mul(self, other: Scalar<N>) -> Vector<N>{
        let dim = self.data.shape().0;
        let mut result = Vector::<N>::new(dim);

        for i in 0..dim{
            result[i] = (self[i] * other[..]).ground_if_zero();
        }

        result
    }
}

impl<N: Numeric> Mul<Scalar<N>> for &Vector<N>{
    type Output = Vector<N>;

    fn mul(self, other: Scalar<N>) -> Vector<N>{
        let dim = self.data.shape().0;
        let mut result = Vector::<N>::new(dim);

        for i in 0..dim{
            result[i] = (self[i] * other[..]).ground_if_zero();
        }

        result
    }
}

impl<N: Numeric> Mul<&Vector<N>> for &Scalar<N>{
    type Output = Vector<N>;

    fn mul(self, other: &Vector<N>) -> Vector<N>{
        let dim = other.data.shape().0;
        let mut result = Vector::<N>::new(dim);

        for i in 0..dim{
            result[i] = (self[..] * other[i]).ground_if_zero();
        }

        result
    }
}

impl<N: Numeric> Mul<Vector<N>> for &Scalar<N>{
    type Output = Vector<N>;

    fn mul(self, other: Vector<N>) -> Vector<N>{
        let dim = other.data.shape().0;
        let mut result = Vector::<N>::new(dim);

        for i in 0..dim{
            result[i] = (self[..] * other[i]).ground_if_zero();
        }

        result
    }
}

impl<N: Numeric> Mul<Vector<N>> for Scalar<N>{
    type Output = Vector<N>;

    fn mul(self, other: Vector<N>) -> Vector<N>{
        let dim = other.data.shape().0;
        let mut result = Vector::<N>::new(dim);

        for i in 0..dim{
            result[i] = (self[..] * other[i]).ground_if_zero();
        }

        result
    }
}

impl<N: Numeric> Mul<&Vector<N>> for Scalar<N>{
    type Output = Vector<N>;

    fn mul(self, other: &Vector<N>) -> Vector<N>{
        let dim = other.data.shape().0;
        let mut result = Vector::<N>::new(dim);

        for i in 0..dim{
            result[i] = (self[..] * other[i]).ground_if_zero();
        }

        result
    }
}


impl<N: Numeric> Mul<N> for &Vector<N>{
    type Output = Vector<N>;

    fn mul(self, other: N) -> Vector<N>{
        let dim = self.data.shape().0;
        let mut result = Vector::<N>::new(dim);

        for i in 0..dim{
            result[i] = (self[i] * other).ground_if_zero();
        }

        result
    }
}

impl<N: Numeric> Mul<N> for Vector<N>{
    type Output = Vector<N>;

    fn mul(self, other: N) -> Vector<N>{
        let dim = self.data.shape().0;
        let mut result = Vector::<N>::new(dim);

        for i in 0..dim{
            result[i] = (self[i] * other).ground_if_zero();
        }

        result
    }
}


// vector-matrix multiplication

impl<N: Numeric> Mul<&Matrix<N>> for &Vector<N>{
    type Output = Vector<N>;

    fn mul(self, other: &Matrix<N>) -> Vector<N>{
        let cols = other.shape().1;

        if cols != self.data.shape().0{
            panic!("Dimension mismatch for vector-matrix multiplication!");
        }

        Vector{
            data: &self.data * other
        }
    }
}

impl<N: Numeric> Mul<&Vector<N>> for &Matrix<N>{
    type Output = Vector<N>;

    fn mul(self, other: &Vector<N>) -> Vector<N>{
        let rows = self.shape().0;

        if rows != other.data.shape().0{
            panic!("Dimension mismatch for matrix-vector multiplication!");
        }

        Vector{
            data: self * &other.data
        }
    }
}

impl<N: Numeric> Mul<Matrix<N>> for Vector<N>{
    type Output = Vector<N>;

    fn mul(self, other: Matrix<N>) -> Vector<N>{
        let cols = other.shape().1;

        if cols != self.data.shape().0{
            panic!("Dimension mismatch for vector-matrix multiplication!");
        }

        Vector{
            data: &self.data * other
        }
    }
}

impl<N: Numeric> Mul<Vector<N>> for Matrix<N>{
    type Output = Vector<N>;

    fn mul(self, other: Vector<N>) -> Vector<N>{
        let rows = self.shape().0;

        if rows != other.data.shape().0{
            panic!("Dimension mismatch for matrix-vector multiplication!");
        }

        Vector{
            data: self * &other.data
        }
    }
}

impl<N: Numeric> Mul<Matrix<N>> for &Vector<N>{
    type Output = Vector<N>;

    fn mul(self, other: Matrix<N>) -> Vector<N>{
        let cols = other.shape().1;

        if cols != self.data.shape().0{
            panic!("Dimension mismatch for vector-matrix multiplication!");
        }

        Vector{
            data: &self.data * other
        }
    }
}

impl<N: Numeric> Mul<Vector<N>> for &Matrix<N>{
    type Output = Vector<N>;

    fn mul(self, other: Vector<N>) -> Vector<N>{
        let rows = self.shape().0;

        if rows != other.data.shape().0{
            panic!("Dimension mismatch for matrix-vector multiplication!");
        }

        Vector{
            data: self * &other.data
        }
    }
}

impl<N: Numeric> Mul<&Matrix<N>> for Vector<N>{
    type Output = Vector<N>;

    fn mul(self, other: &Matrix<N>) -> Vector<N>{
        let cols = other.shape().1;

        if cols != self.data.shape().0{
            panic!("Dimension mismatch for vector-matrix multiplication!");
        }

        Vector{
            data: &self.data * other
        }
    }
}

impl<N: Numeric> Mul<&Vector<N>> for Matrix<N>{
    type Output = Vector<N>;

    fn mul(self, other: &Vector<N>) -> Vector<N>{
        let rows = self.shape().0;

        if rows != other.data.shape().0{
            panic!("Dimension mismatch for matrix-vector multiplication!");
        }

        Vector{
            data: self * &other.data
        }
    }
}


// vector addition
impl<N: Numeric> Add for &Vector<N>{
    type Output = Vector<N>;

    fn add(self, other: &Vector<N>) -> Vector<N>{
        if self.data.shape() != other.data.shape(){
            panic!("Dimension mismatch for vector addition!");
        }

        let dim = self.data.shape().0;
        let mut result = Vector::<N>::new(dim);

        for i in 0..dim{
            result[i] = self[i] + other[i];
        }

        result
    }
}

impl<N: Numeric> Add for Vector<N>{
    type Output = Vector<N>;

    fn add(self, other: Vector<N>) -> Vector<N>{
        if self.data.shape() != other.data.shape(){
            panic!("Dimension mismatch for vector addition!");
        }

        let dim = self.data.shape().0;
        let mut result = Vector::<N>::new(dim);

        for i in 0..dim{
            result[i] = self[i] + other[i];
        }

        result
    }
}

impl<N: Numeric> Add<Vector<N>> for &Vector<N>{
    type Output = Vector<N>;

    fn add(self, other: Vector<N>) -> Vector<N>{
        if self.data.shape() != other.data.shape(){
            panic!("Dimension mismatch for vector addition!");
        }

        let dim = self.data.shape().0;
        let mut result = Vector::<N>::new(dim);

        for i in 0..dim{
            result[i] = self[i] + other[i];
        }

        result
    }
}

impl<N: Numeric> Add<&Vector<N>> for Vector<N>{
    type Output = Vector<N>;

    fn add(self, other: &Vector<N>) -> Vector<N>{
        if self.data.shape() != other.data.shape(){
            panic!("Dimension mismatch for vector addition!");
        }

        let dim = self.data.shape().0;
        let mut result = Vector::<N>::new(dim);

        for i in 0..dim{
            result[i] = self[i] + other[i];
        }

        result
    }
}


impl<N: Numeric> AddAssign<&Vector<N>> for Vector<N>{
    
    fn add_assign(&mut self, other: &Vector<N>){
        if self.data.shape() != other.data.shape(){
            panic!("Dimension mismatch for vector addition!");
        }

        let dim = self.data.shape().0;

        for i in 0..dim{
            self[i] += other[i];
        }
    }
}

impl<N: Numeric> AddAssign<Vector<N>> for Vector<N>{
    fn add_assign(&mut self, other: Vector<N>){
        if self.data.shape() != other.data.shape(){
            panic!("Dimension mismatch for vector addition!");
        }

        let dim = self.data.shape().0;

        for i in 0..dim{
            self[i] += other[i];
        }
    }
}

