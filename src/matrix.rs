use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Range, RangeFull, Sub};
use crate::utils::numbers::Numeric;
use crate::space::Space;
use crate::vector::Vector;

pub struct Matrix<N>{
    data: Vec<N>,
    rows: usize,
    cols: usize,
}

impl<N: Numeric> Matrix<N>{
    pub fn new(rows: usize, cols: usize) -> Self{

        Matrix{
            data: vec![N::zero(); rows * cols],
            rows,
            cols,
        }
    }

    pub fn from_arr(arr: &[N], rows: usize, cols: usize) -> Self{
        Matrix { data: arr.to_vec(), rows, cols }
    }

    pub fn from_vec(vec: Vec<N>, rows: usize, cols: usize) -> Self{
        Matrix { data: vec, rows, cols }
    }

    pub fn from_space(space: &Space<N>, as_col: bool) -> Self{
        if as_col{
            return Matrix::from_columns(&space.vectors);
        }

        Matrix::from_rows(&space.vectors)
    }

    pub fn from_rows(rows: &[Vector<N>]) -> Self{
        let cols = rows[0][..].len();
        let rows_shape = rows.len();
        let mut data = vec![N::zero(); rows_shape * cols];

        for i in 0..rows_shape{
            for j in 0..cols{
                data[i * rows_shape + j] = rows[i][j];
            }
        }

        Matrix { data, rows: rows_shape, cols }
    }

    pub fn from_columns(columns: &[Vector<N>]) -> Self{
        let rows = columns[0][..].len();
        let cols = columns.len();
        let mut data = vec![N::zero(); rows * cols];

        for i in 0..cols{
            for j in 0..rows{
                data[i * cols + j] = columns[i][j];
            }
        }

        Matrix { data, rows, cols }
    }

    pub fn col(&self, index: usize) -> Vector<N>{
        let mut col_vec = vec![N::zero(); self.rows];
        for i in 0..self.rows{
            col_vec[i] = self[i][index];
        }
        Vector::from_arr(&col_vec)
    }

    pub fn with_submatrix(&self, row_start: usize, col_start: usize, submatrix: &Matrix<N>) -> Self{
        let mut result = self.clone();

        for i in 0..submatrix.rows{
            for j in 0..submatrix.cols{
                result[row_start + i][col_start + j] = submatrix[i][j];
            }
        }

        result
    }

    pub fn random() -> Self{
        todo!()
    }

    pub fn diag(arr: &[N]) -> Self{
        let size = arr.len();
        let mut data = vec![N::zero(); size * size];

        for i in 0..size{
            data[i * (size + 1)] = arr[i];
        }

        Matrix { data, rows: size, cols: size }
    }

    pub fn is_diagonal(&self) -> bool{
        for i in 0..self.rows{
            for j in 0..self.cols{
                if i != j && !self[i][j].is_zero(){
                    return false;
                }
            }
        }

        true
    }

    pub fn row_space(&self) -> Space<N>{
        let mut vectors = Vec::<Vector<N>>::new();
        for i in 0..self.rows{
            vectors.push(Vector::from_arr(&self[i]));
        }

        Space::new(vectors)
    }

    pub fn col_space(&self) -> Space<N>{
        let mut vectors = Vec::<Vector<N>>::new();
        for j in 0..self.cols{
            let mut col_vec = vec![N::zero(); self.rows];
            for i in 0..self.rows{
                col_vec[i] = self[i][j];
            }
            vectors.push(Vector::from_arr(&col_vec));
        }

        Space::new(vectors)
    }

    pub fn identity(diag: usize) -> Self{
        let mut data= vec![N::zero(); diag * diag];
        for i in 0..diag{
            data[i * (diag + 1)] = N::one();
        }
        Matrix { data, rows: diag, cols: diag }
    }

    pub fn shape(&self) -> (usize, usize){
        (self.rows, self.cols)
    }

    pub fn transpose(&self) -> Self{
        let mut k = vec![N::zero(); self.rows * self.cols];
        
        for row in 0..self.cols{
            for col in 0..self.rows{
                k[row * self.rows + col] = self[col][row];
            }
        }

        Matrix { data: k, rows: self.cols, cols: self.rows }
    }

    pub fn det(&self) -> N{
        if self.cols != self.rows{
            panic!("Invalid dimension for matrix determinant");
        }

        let mut clone_self = self.clone();
        let mut sign = N::one();
        let mut prev_pivot = N::one();

        //gaussian elimination
        for i in 0..self.cols-1{
            if clone_self[i][i] == N::zero(){
                let mut j = i + 1;
                while j < self.cols && clone_self[j][i] == N::zero() {
                    j += 1;
                }

                if j == self.cols {
                    return N::zero();
                }

                clone_self.swap_rows(i, j);
                sign = sign * N::negative();
            }
            
            let current = clone_self[i][i];

            for j in i + 1..self.rows{
                for k in i + 1..self.cols{
                    clone_self[j][k] = (current * clone_self[j][k] + N::negative() * clone_self[j][i] * clone_self[i][k]) / prev_pivot;
                }
            }

            prev_pivot = current;
        }
        
        sign * clone_self[self.cols - 1][self.cols - 1]
    }

    fn swap_rows(&mut self, row1: usize, row2: usize){
        for i in 0..self.cols{
            self.data.swap(row1 * self.cols + i, row2 * self.cols + i);
        }
    }

    pub fn augment(&self, other: &Matrix<N>) -> Matrix<N>{
        if self.rows != other.rows{
            panic!("Invalid dimension for matrix augmentation");
        }

        let mut data = vec![N::zero(); self.rows * (self.cols + other.cols)];

        for i in 0..self.rows{
            for j in 0..self.cols{
                data[i * (self.cols + other.cols) + j] = self[i][j];
            }

            for j in 0..other.cols{
                data[i * (self.cols + other.cols) + self.cols + j] = other[i][j];
            }
        }

        Matrix { data, rows: self.rows, cols: self.cols + other.cols }
    }

    pub fn dim_truncate(&mut self, rows: usize, cols: usize){
        if rows > self.rows || cols > self.cols{
            panic!("Invalid dimension for matrix truncation");
        }

        
        let mut data = vec![N::zero(); rows * cols];
        for i in 0..rows{
            for j in 0..cols{
                data[i * cols + j] = self[i][j];
            }
        }

        self.data = data;
        self.rows = rows;
        self.cols = cols;
    }

    pub fn inverse(&self) -> Self{
        self.private_inverse(false)
    }

    pub fn safe_inverse(&self) -> Self{
        self.private_inverse(true)
    }

    fn private_inverse(&self, safe: bool) -> Matrix<N>{
        if self.cols != self.rows{
            panic!("Invalid dimension for matrix inverse");
        }

        if self.is_diagonal(){
            let mut data = vec![N::zero(); self.rows * self.cols];
            for i in 0..self.rows{
                if self[i][i].is_zero(){
                    if safe {
                        continue;
                    } else {
                        panic!("Matrix is not invertible");
                    }
                }
                data[i * (self.cols + 1)] = N::one() / self[i][i];
            }
            return Matrix { data, rows: self.rows, cols: self.cols };
        }

        let identity = Matrix::identity(self.cols);
        let mut augmented = self.augment(&identity);

        for i in 0..self.cols{
            let mut pivot_row = i;
            let mut max_val = augmented[i][i].abs();

            for j in (i + 1)..self.cols{
                if augmented[j][i].abs() > max_val {
                    max_val = augmented[j][i].abs();
                    pivot_row = j;
                }
            }

            if max_val <= N::eps() {
                panic!("Cannot calculate matrix inversion");
            }

            if pivot_row != i {
                augmented.swap_rows(i, pivot_row);
            }

            let pivot = augmented[i][i];
            for j in 0..(2 * self.rows) {
                augmented[i][j] = augmented[i][j] / pivot;
            }

            for j in 0..self.rows {
                if i != j { // Skip the pivot row itself
                    let multiplier = augmented[j][i];
                    for k in 0..(2 * self.rows) {
                        augmented[j][k] = augmented[j][k] + N::negative() * multiplier * augmented[i][k];
                    }
                }
            }

        }

        let mut data = vec![N::zero(); self.rows * self.cols];
        
        for i in 0..self.rows{
            for j in 0..self.cols{
                data[i * self.cols + j] = augmented[i][j + self.cols];
            }
        }

        Matrix { data, rows: self.rows, cols: self.cols }
    }

    pub fn scale(mut self, scale_factor: N) -> Self{

        for i in 0..self.rows * self.cols{
            self.data[i] = self.data[i] * scale_factor;
        }

        self
    }

    pub fn print(&self){

        for i in 0..self.rows{
            for j in 0..self.cols{
                print!(" {}", self[i][j]);
            }

            println!("");
        }
    }

    pub fn print_round(&self, decimals: usize){
        for i in 0..self.rows{
            for j in 0..self.cols{
                print!(" {:.decimals$}", self[i][j]);
            }

            println!("");
        }

    }

    pub fn trace(&self) -> N{
        if self.rows != self.cols{
            panic!("Matrix must be square to calculate trace");
        }

        let mut trace = N::zero();
        for i in 0..self.rows{
            trace += self[i][i];
        }

        trace
    }

    pub fn pow(&self, exp: usize) -> Self{
        if self.rows != self.cols{
            panic!("Matrix must be square to calculate power");
        }

        let mut result = self.clone();
        for _ in 1..exp{
           result = &result * self;
        }

        result
    }

    pub fn forbenius_sq_norm(&self) -> N{
        let mut sum = N::zero();
        for i in 0..self.rows * self.cols{
            sum += self.data[i] * self.data[i];
        }

        sum
    }

    pub fn forbenius_norm(&self) -> N{
        self.forbenius_sq_norm().sqrt()
    }

    pub fn mean(&self) -> N{
        let mut sum = N::zero();
        for i in 0..self.rows * self.cols{
            sum += self.data[i];
        }

        N::from_float(sum.to_float() / (self.rows * self.cols) as f64)
    }

    pub fn center_matrix(&self) -> Self{
        let mean = self.mean();
        let mut result = self.clone();
        for i in 0..result.rows * result.cols{
            result.data[i] = result.data[i] + N::negative() * mean;
        }

        result
    }

    pub fn mean_axis(&self, axis: usize) -> Vector<N>{
        if axis == 0{
            let mut mean_vec = vec![N::zero(); self.cols];
            for i in 0..self.rows{
                for j in 0..self.cols{
                    mean_vec[j] += self[i][j];
                }
            }

            for j in 0..self.cols{
                mean_vec[j] = mean_vec[j] / N::from_float(self.rows as f64);
            }

            Vector::from_arr(&mean_vec)
        }else if axis == 1{
            let mut mean_vec = vec![N::zero(); self.rows];
            for i in 0..self.rows{
                for j in 0..self.cols{
                    mean_vec[i] += self[i][j];
                }
            }

            for i in 0..self.rows{
                mean_vec[i] = mean_vec[i] / N::from_float(self.cols as f64);
            }

            Vector::from_arr(&mean_vec)
        }else{
            panic!("Invalid axis for mean calculation");
        }
    }

    pub fn center_matrix_axis(&self, axis: usize) -> Self{
        let mean_vec = self.mean_axis(axis);
        let mut result = self.clone();

        if axis == 0{
            for i in 0..result.rows{
                for j in 0..result.cols{
                    result[i][j] = result[i][j] + N::negative() * mean_vec[j];
                }
            }
        }else if axis == 1{
            for i in 0..result.rows{
                for j in 0..result.cols{
                    result[i][j] = result[i][j] + N::negative() * mean_vec[i];
                }
            }
        }else{
            panic!("Invalid axis for centering");
        }

        result
    }

    pub fn gauss_elim(&self) -> Self{
        let mut clone_self = self.clone();

        for i in 0..self.cols - 1{
            for j in i + 1..self.rows{
                let multiplier = clone_self[j][i] / clone_self[i][i];
                for k in i..self.cols{
                    let temp = clone_self[j][k] + N::negative() * multiplier * clone_self[i][k];
                    clone_self[j][k] = if temp.abs() <= N::eps(){ N::zero() }else{ temp };
                }
            }
        }

        clone_self
    }

    pub fn rank(&self) -> usize{
        let mut rank = 0;
        let clone_self = self.gauss_elim();

        for i in 0..clone_self.rows{
            let mut non_zero_row = false;
            for j in 0..clone_self.cols{
                if clone_self[i][j].abs() > N::eps(){
                    non_zero_row = true;
                    break;
                }
            }

            if non_zero_row{
                rank += 1;
            }
        }

        rank
    }

    pub fn pseudo_inverse(&self) -> Self{
        &(&self.transpose() * self).inverse() * &self.transpose()
    }

    pub fn main_diag(&self) -> Vec<N>{
        let mut diag = vec![N::zero(); self.rows.min(self.cols)];
        for i in 0..diag.len(){
            diag[i] = self[i][i];
        }
        diag
    }

    pub fn reshape(&mut self, rows: usize, cols: usize) {
        if rows * cols != self.rows * self.cols {
            panic!("Incompatible matrix dimensions");
        }

        self.rows = rows;
        self.cols = cols;
    }
}

impl<N: Numeric> Iterator for Matrix<N>{
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty(){
            None
        }else{
            Some(self.data.remove(0))
        }
    }
}

impl<N: Numeric> FromIterator<N> for Matrix<N>{
    fn from_iter<T: IntoIterator<Item = N>>(iter: T) -> Matrix<N> {
        let data: Vec<N> = iter.into_iter().collect();
        let len = data.len();

        Matrix{
            data,
            rows: len,
            cols: 1,
        }
    }
}

impl<N: Numeric> Clone for Matrix<N>{
    fn clone(&self) -> Self {
        Matrix { data: self.data.clone(), rows: self.rows, cols: self.cols }
    }
}

impl<N: Numeric> Index<usize> for Matrix<N>{
    type Output = [N];

    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.cols;
        &self.data[start..start + self.cols]
    }
}

impl<N: Numeric> IndexMut<usize> for Matrix<N>{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * self.cols;
        &mut self.data[start..start + self.cols]
    }
}

impl<N: Numeric> Index<Range<usize>> for Matrix<N>{
    type Output = [N];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        let start = index.start * self.cols;
        let end = index.end * self.cols;
        &self.data[start..end]
    }
}

impl<N: Numeric> Index<RangeFull> for Matrix<N>{
    type Output = [N];

    fn index(&self, _: RangeFull) -> &Self::Output {
        &self.data[..]
    }
}

impl<N: Numeric> Add for &Matrix<N>{
    type Output = Matrix<N>;

    fn add(self, other: &Matrix<N>) -> Matrix<N>{
        if (self.cols != other.cols) || (self.rows != other.rows){
            panic!("Matrix must have the same dimension!");
        }

        let rows = self.rows;
        let cols = self.cols;
        let data = self
            .data.clone()
            .into_iter()
            .zip(other.data.clone().into_iter())
            .map(|(a, b)| a + b)
            .collect();

        Matrix { data, rows, cols }
    }
}

impl<N: Numeric> Add for Matrix<N>{
    type Output = Matrix<N>;

    fn add(self, other: Matrix<N>) -> Matrix<N>{
        if (self.cols != other.cols) || (self.rows != other.rows){
            panic!("Matrix must have the same dimension!");
        }

        let rows = self.rows;
        let cols = self.cols;
        let data = self
            .data.clone()
            .into_iter()
            .zip(other.data.into_iter())
            .map(|(a, b)| a + b)
            .collect();

        Matrix { data, rows, cols }
    }
}

impl<N: Numeric> Add<Matrix<N>> for &Matrix<N>{
    type Output = Matrix<N>;

    fn add(self, other: Matrix<N>) -> Matrix<N>{
        if (self.cols != other.cols) || (self.rows != other.rows){
            panic!("Matrix must have the same dimension!");
        }

        let rows = self.rows;
        let cols = self.cols;
        let data = self
            .data.clone()
            .into_iter()
            .zip(other.data.into_iter())
            .map(|(a, b)| a + b)
            .collect();

        Matrix { data, rows, cols }
    }
}

impl<N: Numeric> Add<&Matrix<N>> for Matrix<N>{
    type Output = Matrix<N>;

    fn add(self, other: &Matrix<N>) -> Matrix<N>{
        if (self.cols != other.cols) || (self.rows != other.rows){
            panic!("Matrix must have the same dimension!");
        }

        let rows = self.rows;
        let cols = self.cols;
        let data = self
            .data.clone()
            .into_iter()
            .zip(other.data.clone().into_iter())
            .map(|(a, b)| a + b)
            .collect();

        Matrix { data, rows, cols }
    }
}

impl<N: Numeric> Sub for &Matrix<N>{
    type Output = Matrix<N>;

    fn sub(self, other: &Matrix<N>) -> Matrix<N>{
        if (self.cols != other.cols) || (self.rows != other.rows){
            panic!("Matrix must have the same dimension!");
        }

        let rows = self.rows;
        let cols = self.cols;
        let data = self
            .data.clone()
            .into_iter()
            .zip(other.data.clone().into_iter())
            .map(|(a, b)| a + N::negative() * b)
            .collect();

        Matrix { data, rows, cols }
    }
}

impl<N: Numeric> Sub for Matrix<N>{
    type Output = Matrix<N>;

    fn sub(self, other: Matrix<N>) -> Matrix<N>{
        if (self.cols != other.cols) || (self.rows != other.rows){
            panic!("Matrix must have the same dimension!");
        }

        let rows = self.rows;
        let cols = self.cols;
        let data = self
            .data
            .into_iter()
            .zip(other.data.into_iter())
            .map(|(a, b)| a + N::negative() * b)
            .collect();

        Matrix { data, rows, cols }
    }
}

impl<N: Numeric> Sub<Matrix<N>> for &Matrix<N>{
    type Output = Matrix<N>;

    fn sub(self, other: Matrix<N>) -> Matrix<N>{
        if (self.cols != other.cols) || (self.rows != other.rows){
            panic!("Matrix must have the same dimension!");
        }

        let rows = self.rows;
        let cols = self.cols;
        let data = self
            .data.clone()
            .into_iter()
            .zip(other.data.into_iter())
            .map(|(a, b)| a + N::negative() * b)
            .collect();

        Matrix { data, rows, cols }
    }
}

impl<N: Numeric> Sub<&Matrix<N>> for Matrix<N>{
    type Output = Matrix<N>;

    fn sub(self, other: &Matrix<N>) -> Matrix<N>{
        if (self.cols != other.cols) || (self.rows != other.rows){
            panic!("Matrix must have the same dimension!");
        }

        let rows = self.rows;
        let cols = self.cols;
        let data = self
            .data
            .into_iter()
            .zip(other.data.clone().into_iter())
            .map(|(a, b)| a + N::negative() * b)
            .collect();

        Matrix { data, rows, cols }
    }
}

impl<N: Numeric> Mul for &Matrix<N>{
    type Output = Matrix<N>;

    fn mul(self, other: &Matrix<N>) -> Matrix<N>{

        let (sr, sc) = self.shape();
        let (or, oc) = other.shape();
        
        // sr * sc  x  or * oc = sr * oc--> sc == or
        if sc != or{
            panic!("Invalid dimension when performing matrix multiplication");
        }

        let mut data = vec![N::zero(); sr * oc];
        
        for i in 0..sr{
            
            for j in 0..oc{
                let mut total = N::zero();

                //same for 0..sc
                for k in 0..or{
                    total += (self[i][k] * other[k][j]).ground_if_zero();
                }
                
                data[i * oc + j] = total.ground_if_zero();
            }
        }

        Matrix { data, rows: sr, cols: oc }
    }

}

impl<N: Numeric> Mul<Matrix<N>> for &Matrix<N>{
    type Output = Matrix<N>;

    fn mul(self, other: Matrix<N>) -> Matrix<N>{

        let (sr, sc) = self.shape();
        let (or, oc) = other.shape();
        
        // sr * sc  x  or * oc = sr * oc--> sc == or
        if sc != or{
            panic!("Invalid dimension when performing matrix multiplication");
        }

        let mut data = vec![N::zero(); sr * oc];
        
        for i in 0..sr{
            
            for j in 0..oc{
                let mut total = N::zero();

                //same for 0..sc
                for k in 0..or{
                    total += (self[i][k] * other[k][j]).ground_if_zero();
                }
                
                data[i * oc + j] = total.ground_if_zero();
            }
        }

        Matrix { data, rows: sr, cols: oc }
    }
}

impl<N: Numeric> Mul for Matrix<N>{
    type Output = Matrix<N>;

    fn mul(self, other: Matrix<N>) -> Matrix<N>{

        let (sr, sc) = self.shape();
        let (or, oc) = other.shape();
        
        // sr * sc  x  or * oc = sr * oc--> sc == or
        if sc != or{
            panic!("Invalid dimension when performing matrix multiplication");
        }

        let mut data = vec![N::zero(); sr * oc];
        
        for i in 0..sr{
            
            for j in 0..oc{
                let mut total = N::zero();

                //same for 0..sc
                for k in 0..or{
                    total += (self[i][k] * other[k][j]).ground_if_zero();
                }
                
                data[i * oc + j] = total.ground_if_zero();
            }
        }

        Matrix { data, rows: sr, cols: oc }
    }

}

impl<N: Numeric> Mul<&Matrix<N>> for Matrix<N>{
    type Output = Matrix<N>;

    fn mul(self, other: &Matrix<N>) -> Matrix<N>{

        let (sr, sc) = self.shape();
        let (or, oc) = other.shape();
        
        // sr * sc  x  or * oc = sr * oc--> sc == or
        if sc != or{
            panic!("Invalid dimension when performing matrix multiplication");
        }

        let mut data = vec![N::zero(); sr * oc];
        
        for i in 0..sr{
            
            for j in 0..oc{
                let mut total = N::zero();

                //same for 0..sc
                for k in 0..or{
                    total += (self[i][k] * other[k][j]).ground_if_zero();
                }
                
                data[i * oc + j] = total.ground_if_zero();
            }
        }

        Matrix { data, rows: sr, cols: oc }
    }
    
}

impl<N: Numeric> AddAssign<&Matrix<N>> for Matrix<N>{
    fn add_assign(&mut self, other: &Matrix<N>) {
        if (self.cols != other.cols) || (self.rows != other.rows){
            panic!("Matrix must have the same dimension!");
        }

        for i in 0..self.rows * self.cols{
            self.data[i] = self.data[i] + other.data[i];
        }
    }
}

impl<N: Numeric> AddAssign<Matrix<N>> for Matrix<N>{
    fn add_assign(&mut self, other: Matrix<N>) {
        if (self.cols != other.cols) || (self.rows != other.rows){
            panic!("Matrix must have the same dimension!");
        }

        for i in 0..self.rows * self.cols{
            self.data[i] = self.data[i] + other.data[i];
        }
    }
}