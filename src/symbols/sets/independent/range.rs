use super::into_set::IntoSet;
use alloc::vec::Vec;
use core::ops::Range;

impl IntoSet for Range<usize> {
    type SetGen = Vec<usize>;

    fn into_set(self) -> Self::SetGen {
        self.collect()
    }
}
