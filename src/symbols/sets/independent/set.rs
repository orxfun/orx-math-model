use super::into_set::IntoSet;
use alloc::collections::BTreeSet;
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::collections::HashSet;

// usize

#[cfg(feature = "std")]
impl IntoSet for HashSet<usize> {
    type SetGen = Vec<usize>;

    fn into_set(self) -> Self::SetGen {
        self.into_iter().collect()
    }
}

#[cfg(feature = "std")]
impl IntoSet for &HashSet<usize> {
    type SetGen = Vec<usize>;

    fn into_set(self) -> Self::SetGen {
        self.iter().copied().collect()
    }
}

#[cfg(feature = "std")]
impl IntoSet for BTreeSet<usize> {
    type SetGen = Vec<usize>;

    fn into_set(self) -> Self::SetGen {
        self.into_iter().collect()
    }
}

#[cfg(feature = "std")]
impl IntoSet for &BTreeSet<usize> {
    type SetGen = Vec<usize>;

    fn into_set(self) -> Self::SetGen {
        self.iter().copied().collect()
    }
}
