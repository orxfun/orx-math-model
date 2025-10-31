use crate::symbols::sets::indices::Depth;
use alloc::vec::Vec;
use core::ops::Index;

pub struct IndexValues {
    indices: Vec<usize>,
}

impl IndexValues {
    pub fn values<const N: usize>(&self, depths: [Depth; N]) -> [usize; N] {
        depths.map(|d| self.indices[d.depth()])
    }
}

impl Index<Depth> for IndexValues {
    type Output = usize;

    fn index(&self, index: Depth) -> &Self::Output {
        &self.indices[index.depth()]
    }
}
