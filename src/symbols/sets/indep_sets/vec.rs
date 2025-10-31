use crate::symbols::sets::{
    elements::Elements,
    set_gen::{IntoSetGen, SetGen},
};
use alloc::vec::Vec;

// usize

impl SetGen for Vec<usize> {
    fn elements(&self, _: usize, _: &mut Elements) -> Option<&[usize]> {
        Some(&self)
    }
}

impl IntoSetGen for Vec<usize> {
    type SetGen = Self;

    fn into_set_gen(self) -> Self::SetGen {
        self
    }
}

// string
