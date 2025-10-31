use crate::symbols::sets::indices::{Depth, Elements, IndexValues, SetDepths};
use crate::symbols::{sets::set_gen::SetGen, Set};

pub struct DependentSubset<'m, F>
where
    F: Fn(usize, usize) -> bool,
{
    parent: Set<'m>,
    filter: F,
}

impl<'m, F> DependentSubset<'m, F>
where
    F: Fn(usize, usize) -> bool,
{
    pub(crate) fn new(parent: Set<'m>, filter: F) -> Self {
        Self { parent, filter }
    }
}

impl<'m, F> SetGen<'m> for DependentSubset<'m, F>
where
    F: Fn(usize, usize) -> bool,
{
    fn set_elements(
        &'m self,
        depth: Depth,
        set_depths: SetDepths<'_>,
        index_values: &IndexValues,
        elements: &'m mut Elements<'m>,
    ) {
        let parent_depth = set_depths.depth_of(self.parent);
        let parent_elements = elements.parent_elements(depth, parent_depth);

        let parent_depth = set_depths.depth_of(self.parent);
        let i1 = index_values[parent_depth];

        let i2 = parent_elements
            .iter()
            .copied()
            .filter(|i2| (self.filter)(i1, *i2));
        elements.set_stored_elements(depth, i2);
    }
}
