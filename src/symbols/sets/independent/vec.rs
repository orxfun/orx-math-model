use crate::symbols::sets::index_values::IndexValues;
use crate::symbols::sets::set_gen::{IntoSetGen, SetGen};
use alloc::vec::Vec;

// usize

impl SetGen for Vec<usize> {
    fn elements(&self, _: &mut IndexValues) -> Option<&[usize]> {
        Some(&self)
    }
}

impl IntoSetGen for Vec<usize> {
    type SetGen = Self;

    fn into_set_gen(self) -> Self::SetGen {
        self
    }
}
