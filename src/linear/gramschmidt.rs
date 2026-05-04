use crate::vector::Vector;
use crate::space::Space;
use crate::utils::numbers::Numeric;

pub fn gramschmidt<N: Numeric>(vectors: &Space<N>) -> Space<N>{
    let mut orthogonal_vectors = Vec::<Vector<N>>::new();

    for i in 0..vectors.len(){
        let vk = &vectors[i];
        let mut w = vk.clone();
   
        for u in &orthogonal_vectors{

            w += vk.proj_to(u) * N::negative();
        }

        orthogonal_vectors.push(w.normalize());
    }

    Space::new(orthogonal_vectors)
}