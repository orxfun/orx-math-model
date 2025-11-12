use crate::data::set_data::indices::{IndexValues, SetDepths};
use crate::Model;

pub struct IndexValuesIter<'m> {
    model: &'m Model,
    depending_indices: &'m [usize],
    depths: &'m SetDepths<'m>,
    index_values: &'m IndexValues,
    i: usize,
}

impl<'m> IndexValuesIter<'m> {
    pub fn new(
        model: &'m Model,
        depending_indices: &'m [usize],
        depths: &'m SetDepths<'m>,
        index_values: &'m IndexValues,
    ) -> Self {
        Self {
            model,
            depending_indices,
            depths,
            index_values,
            i: 0,
        }
    }
}

impl<'m> Iterator for IndexValuesIter<'m> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.depending_indices.get(self.i);
        let dep_set = idx.map(|idx| self.model.set_at_unchecked(*idx))?;
        self.i += 1;
        let depth = self.depths.depth_of(dep_set);
        Some(self.index_values[depth])
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.depending_indices.len();
        let remaining = len - self.i;
        (remaining, Some(remaining))
    }
}

impl<'m> ExactSizeIterator for IndexValuesIter<'m> {}
