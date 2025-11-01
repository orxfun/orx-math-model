use crate::symbols::sets::indices::{Depth, Elements, IndexValues, SetDepths};
use crate::symbols::{sets::set_gen::SetGen, Set};

pub struct Subset<'m, F>
where
    F: Fn(usize, usize) -> bool,
{
    set: Set<'m>,
    filter: F,
}

impl<F> SetGen for Subset<'_, F>
where
    F: Fn(usize, usize) -> bool,
{
    fn set_elements<'m>(
        &'m self,
        depth: Depth,
        set_depths: SetDepths<'_>,
        index_values: &IndexValues,
        elements: &'m mut Elements<'m>,
    ) {
        let set_depth = set_depths.depth_of(self.set);
        let set = elements.parent_elements(depth, set_depth);
        let subset = set.iter().filter(|x| (self.filter)(**x)).copied();
        elements.set_stored_elements(depth, subset);
    }
}
