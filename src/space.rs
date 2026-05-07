use crate::vector::Vector;
use crate::matrix::Matrix;
use crate::utils::numbers::Numeric;
use std::ops::{Index, IndexMut};
use crate::linear::gramschmidt::gramschmidt;

pub struct Space<N>{
    pub vectors: Vec<Vector<N>>,
}

impl<N: Numeric> Space<N>{
    pub fn new(vectors: Vec<Vector<N>>) -> Self{
        Self{vectors}
    }

    pub fn to_matrix(&self) -> Matrix<N>{
        Matrix::from_space(&self, true)
    }

    pub fn is_basis(&self) -> bool{
        self.to_matrix().det() != N::zero()
    }

    pub fn dim(&self) -> usize{
        self.to_matrix().rank()
    }

    pub fn empty() -> Self{
        Self{vectors: Vec::new()}
    }

    pub fn append(&mut self, vector: Vector<N>){
        self.vectors.push(vector);
    }

    pub fn orthogonize(&mut self){
        *self = gramschmidt(self);
    }

    pub fn len(&self) -> usize{
        self.vectors.len()
    }
}

impl<N: Numeric> Index<usize> for Space<N>{
    type Output = Vector<N>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vectors[index]
    }
    
}

impl<N: Numeric> IndexMut<usize> for Space<N>{

    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vectors[index]
    }
    
}
