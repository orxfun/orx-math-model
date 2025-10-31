use crate::symbols::sets::indices::{Depth, Elements, IndexValues, SetDepths};
use crate::symbols::{sets::set_gen::SetGen, Set};

pub struct DependentSet<'m, const N: usize, I, E>
where
    I: IntoIterator<Item = usize>,
    E: Fn([usize; N]) -> I,
{
    parents: [Set<'m>; N],
    elements: E,
}

impl<'m, const N: usize, I, E> SetGen<'m> for DependentSet<'m, N, I, E>
where
    I: IntoIterator<Item = usize>,
    E: Fn([usize; N]) -> I,
{
    fn set_elements(
        &'m self,
        depth: Depth,
        set_depths: SetDepths<'_>,
        index_values: &IndexValues,
        elements: &'m mut Elements<'m>,
    ) {
        let parent_depths = self.parents.map(|p| set_depths.depth_of(p));
        let parent_indices = index_values.values(parent_depths);
        let dep_elements = (self.elements)(parent_indices);
        elements.set_stored_elements(depth, dep_elements);
    }
}
