use crate::data::set_data::indices::{IndexValues, SetDepths};
use crate::symbols::sets::SetCore;
use crate::Set;
use alloc::boxed::Box;

pub trait SetGen<const N: usize> {
    fn elements_by_dependencies(
        &self,
        depending_indices: [usize; N],
    ) -> Box<dyn Iterator<Item = usize> + '_>;

    fn elements<'m>(
        &self,
        set: Set<'m, N>,
        depths: &SetDepths<'m>,
        index_values: &IndexValues,
    ) -> Box<dyn Iterator<Item = usize> + '_> {
        let dep_sets = set.depending_sets();
        let depths = dep_sets.map(|s| depths.depth_of(s.into()));
        let depending_indices = depths.map(|d| index_values[d]);
        self.elements_by_dependencies(depending_indices)
    }

    fn elements2<'m>(
        &self,
        set: SetCore<'m>,
        depths: &SetDepths<'m>,
        index_values: &IndexValues,
    ) -> Box<dyn Iterator<Item = usize> + '_> {
        let dep_sets = set.depending_sets_core();
        let depths = dep_sets.map(|s| depths.depth_of(s));
        let depending_indices = depths.map(|d| index_values[d]);
        // self.elements_by_dependencies(depending_indices)
        todo!()
    }
}
