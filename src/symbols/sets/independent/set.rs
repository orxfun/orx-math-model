use super::into_set::IntoSet;
use alloc::vec::Vec;
use std::collections::HashSet;

// usize

impl IntoSet for HashSet<usize> {
    type SetGen<'m> = Vec<usize>;

    fn into_set_gen<'m>(self) -> Self::SetGen<'m> {
        self.into_iter().collect()
    }
}

impl IntoSet for &HashSet<usize> {
    type SetGen<'m> = Vec<usize>;

    fn into_set_gen<'m>(self) -> Self::SetGen<'m> {
        self.iter().copied().collect()
    }
}
