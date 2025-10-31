use super::into_set::IntoSet;
use alloc::vec::Vec;
use core::ops::Range;

impl<T> IntoSet for Range<T>
where
    usize: From<T>,
    Range<T>: Iterator<Item = T>,
{
    type SetGen = Vec<usize>;

    fn into_set_gen(self) -> Self::SetGen {
        self.map(usize::from).collect()
    }
}
