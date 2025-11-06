use super::Depth;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Index;

pub struct IndexValues {
    indices: Vec<usize>,
}

impl IndexValues {
    pub fn new(max_depth: usize) -> Self {
        Self {
            indices: vec![usize::MAX; max_depth],
        }
    }
}

impl Index<Depth> for IndexValues {
    type Output = usize;

    fn index(&self, index: Depth) -> &Self::Output {
        &self.indices[index.depth()]
    }
}
