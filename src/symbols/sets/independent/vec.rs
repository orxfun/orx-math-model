use crate::symbols::sets::indices::{Depth, Elements, IndexValues};
use crate::symbols::sets::set_gen::{IntoSetGen, SetGen};
use alloc::vec::Vec;

// usize

impl SetGen for Vec<usize> {
    fn set_elements<'a>(&'a self, depth: Depth, _: &IndexValues, elements: &'a mut Elements<'a>) {
        elements.set_independent_elements(depth, &self);
    }
}

impl IntoSetGen for Vec<usize> {
    type SetGen = Self;

    fn into_set_gen(self) -> Self::SetGen {
        self
    }
}
