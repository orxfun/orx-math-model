use crate::symbols::sets::indices::{Depth, Elements, IndexValues, SetDepths};
use crate::symbols::sets::set_gen::{IntoSetGen, SetGen};
use alloc::vec::Vec;

// usize

impl SetGen for Vec<usize> {
    fn set_elements<'m>(
        &'m self,
        depth: Depth,
        _: SetDepths<'_>,
        _: &IndexValues,
        elements: &'m mut Elements<'m>,
    ) {
        elements.set_independent_elements(depth, &self);
    }
}

impl IntoSetGen for Vec<usize> {
    type SetGen = Self;

    fn into_set_gen(self) -> Self::SetGen {
        self
    }
}
