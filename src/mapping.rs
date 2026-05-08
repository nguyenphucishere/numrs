use crate::matrix::Matrix;
use crate::utils::numbers::Numeric;
use core::marker::PhantomData;

struct LinearMapping<N: Numeric, F: Fn(Matrix<N>) -> Matrix<N>> {
    map: F,
    _marker: PhantomData<N>,
}

impl<N: Numeric, F: Fn(Matrix<N>) -> Matrix<N>> LinearMapping<N, F> {
    pub fn new(map: F) -> Self {
        LinearMapping {
            map,
            _marker: PhantomData,
        }
    }

    pub fn map_to(&self, input: Matrix<N>) -> Matrix<N> {
        (self.map)(input)
    }

    pub fn kernel(&self) -> Vec<Matrix<N>> {
        
        todo!()
    }

    pub fn image(&self) -> Vec<Matrix<N>> {
        todo!()
    }

    pub fn is_injective(&self) -> bool {
        self.kernel().is_empty()
    }

    pub fn is_surjective(&self) -> bool {
        // This is a placeholder. In a real implementation, we would need to check if the image spans the codomain.
        !self.image().is_empty()
    }

    pub fn is_bijective(&self) -> bool {
        self.is_injective() && self.is_surjective()
    }

    pub fn compose<G: Fn(Matrix<N>) -> Matrix<N>>(&self, other: LinearMapping<N, G>) -> LinearMapping<N, impl Fn(Matrix<N>) -> Matrix<N>> {
        let composed_map = move |x: Matrix<N>| {
            let intermediate = (other.map)(x);
            (self.map)(intermediate)
        };
        LinearMapping::new(composed_map)
    }

    pub fn domain(&self) -> Vec<Matrix<N>> {
        // This is a placeholder. In a real implementation, we would need to define the domain of the mapping.
        vec![]
    }

    pub fn codomain(&self) -> Vec<Matrix<N>> {
        // This is a placeholder. In a real implementation, we would need to define the codomain of the mapping.
        vec![]
    }

    pub fn rank(&self) -> usize {
        self.image().len()
    }

    pub fn nullity(&self) -> usize {
        self.kernel().len()
    }

    pub fn rank_nullity_theorem(&self) -> bool {
        self.rank() + self.nullity() == self.domain().len()
    }

    pub fn is_isomorphism(&self) -> bool {
        self.is_bijective()
    }

    pub fn is_endomorphism(&self) -> bool {
        // Check if the domain and codomain are the same.
        todo!()
    }

    pub fn is_automorphism(&self) -> bool {
        self.is_isomorphism() && self.is_endomorphism()
    }

    // TODO: add methods for inverse, etc.
}