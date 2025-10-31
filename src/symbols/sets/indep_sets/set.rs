use crate::symbols::sets::set_gen::IntoSetGen;
use alloc::vec::Vec;
use std::collections::HashSet;

// usize

impl IntoSetGen for HashSet<usize> {
    type SetGen = Vec<usize>;

    fn into_set_gen(self) -> Self::SetGen {
        self.into_iter().collect()
    }
}

impl IntoSetGen for &HashSet<usize> {
    type SetGen = Vec<usize>;

    fn into_set_gen(self) -> Self::SetGen {
        self.iter().copied().collect()
    }
}
