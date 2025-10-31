use crate::symbols::sets::set_gen::IntoSetGen;
use alloc::vec::Vec;
use std::collections::HashSet;

// usize

impl IntoSetGen for HashSet<usize> {
    type SetGen<'m> = Vec<usize>;

    fn into_set_gen<'m>(self) -> Self::SetGen<'m> {
        self.into_iter().collect()
    }
}

impl IntoSetGen for &HashSet<usize> {
    type SetGen<'m> = Vec<usize>;

    fn into_set_gen<'m>(self) -> Self::SetGen<'m> {
        self.iter().copied().collect()
    }
}
