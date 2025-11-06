use crate::symbols::sets::{Set, SetCore};
use alloc::vec::Vec;

pub struct DependentSetIndices {
    set_indices: Vec<usize>,
}

impl DependentSetIndices {
    pub fn new<'m>(sets: impl Iterator<Item = Set<'m, 0>>) -> Self {
        Self {
            set_indices: sets.map(|s| s.idx()).collect(),
        }
    }

    pub fn push(&mut self, set: SetCore<'_>) {
        let idx = set.idx();
        if !self.set_indices.contains(&idx) {
            self.set_indices.push(idx);
        }
        self.set_indices.sort();
    }

    pub fn indices(&self) -> &[usize] {
        &self.set_indices
    }

    pub fn dim(&self) -> usize {
        self.set_indices.len()
    }
}
