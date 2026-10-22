use super::Depth;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::{Index, IndexMut};

pub struct IndexValues {
    indices: Vec<usize>,
}

impl IndexValues {
    pub fn new(max_depth: Depth) -> Self {
        Self {
            indices: vec![usize::MAX; max_depth.depth()],
        }
    }
}

impl Index<Depth> for IndexValues {
    type Output = usize;

    fn index(&self, index: Depth) -> &Self::Output {
        &self.indices[index.depth()]
    }
}

impl IndexMut<Depth> for IndexValues {
    fn index_mut(&mut self, index: Depth) -> &mut Self::Output {
        &mut self.indices[index.depth()]
    }
}
