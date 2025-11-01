use crate::symbols::Set;
use alloc::vec::Vec;

#[derive(Default)]
pub struct DependentSetIndices {
    set_indices: Vec<usize>,
}

impl DependentSetIndices {
    pub fn push(&mut self, set: Set<'_>) {
        let idx = set.idx();
        if !self.set_indices.contains(&idx) {
            self.set_indices.push(idx);
        }
        self.set_indices.sort();
    }

    pub fn indices(&self) -> &[usize] {
        &self.set_indices
    }
}
