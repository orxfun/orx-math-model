use crate::data::set_data::indices::{IndexValues, SetDepths};
use crate::symbols::sets::SetCore;

pub struct IndexValuesIter<'m> {
    set: SetCore<'m>,
    depths: &'m SetDepths<'m>,
    index_values: &'m IndexValues,
    i: usize,
}

impl<'m> Iterator for IndexValuesIter<'m> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let dep_set = self.set.depending_set_core_at(self.i)?;
        self.i += 1;
        let depth = self.depths.depth_of(dep_set);
        Some(self.index_values[depth])
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.set.dim();
        (len, Some(len))
    }
}

impl<'m> ExactSizeIterator for IndexValuesIter<'m> {}
