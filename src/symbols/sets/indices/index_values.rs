use crate::symbols::sets::indices::Depth;
use alloc::vec::Vec;
use core::ops::Index;

pub struct IndexValues {
    indices: Vec<usize>,
}

impl Index<Depth> for IndexValues {
    type Output = usize;

    fn index(&self, index: Depth) -> &Self::Output {
        &self.indices[index.depth()]
    }
}
