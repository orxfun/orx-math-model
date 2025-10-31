use crate::symbols::sets::set_gen::IntoSetGen;
use alloc::vec::Vec;
use core::ops::Range;

impl<T> IntoSetGen for Range<T>
where
    usize: From<T>,
    Range<T>: Iterator<Item = T>,
{
    type SetGen = Vec<usize>;

    fn into_set_gen(self) -> Self::SetGen {
        self.map(usize::from).collect()
    }
}
