use crate::symbols::sets::indices::{Depth, Elements, IndexValues, SetDepths};
use crate::symbols::{sets::set_gen::SetGen, Set};

pub struct Subset<'m, F>
where
    F: Fn(usize) -> bool,
{
    set: Set<'m>,
    filter: F,
}

impl<F> SetGen for Subset<'_, F>
where
    F: Fn(usize) -> bool,
{
    fn set_elements<'m>(
        &'m self,
        depth: Depth,
        set_depths: SetDepths<'_>,
        current_indices: &IndexValues,
        elements: &'m mut Elements<'m>,
    ) {
    }
}
