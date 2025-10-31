use super::into_set::IntoSet;
use alloc::vec::Vec;
use std::collections::HashSet;

// usize

impl IntoSet for HashSet<usize> {
    type SetGen = Vec<usize>;

    fn into_set_gen(self) -> Self::SetGen {
        self.into_iter().collect()
    }
}

impl IntoSet for &HashSet<usize> {
    type SetGen = Vec<usize>;

    fn into_set_gen(self) -> Self::SetGen {
        self.iter().copied().collect()
    }
}
