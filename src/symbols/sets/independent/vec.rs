use super::into_set::IntoSet;
use crate::symbols::sets::indices::{Depth, Elements, IndexValues, SetDepths};
use crate::symbols::sets::set_gen::SetGen;
use alloc::vec::Vec;

// usize

impl SetGen for Vec<usize> {
    fn set_elements<'m>(
        &'m self,
        depth: Depth,
        _: SetDepths<'_>,
        _: &IndexValues,
        elements: &mut Elements<'m>,
    ) {
        elements.set_independent_elements(depth, &self);
    }
}

impl IntoSet for Vec<usize> {
    type SetGen = Self;

    fn into_set(self) -> Self::SetGen {
        self
    }
}
