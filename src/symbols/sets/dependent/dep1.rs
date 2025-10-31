use crate::symbols::sets::indices::{Depth, Elements, IndexValues, SetDepths};
use crate::symbols::{sets::set_gen::SetGen, Set};

pub struct DepSet1<'m, I, E>
where
    I: IntoIterator<Item = usize>,
    E: Fn(usize) -> I,
{
    parent: Set<'m>,
    elements: E,
}

impl<'m, I, E> SetGen<'m> for DepSet1<'m, I, E>
where
    I: IntoIterator<Item = usize>,
    E: Fn(usize) -> I,
{
    fn set_elements(
        &'m self,
        depth: Depth,
        set_depths: SetDepths<'_>,
        index_values: &IndexValues,
        elements: &'m mut Elements<'m>,
    ) {
        let parent_depth = set_depths.depth_of(self.parent);
        let parent_value = index_values[parent_depth];
        let dep_elements = (self.elements)(parent_value);
        elements.set_stored_elements(depth, dep_elements);
    }
}
