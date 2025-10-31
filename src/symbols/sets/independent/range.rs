use crate::symbols::sets::set_gen::IntoSetGen;
use alloc::vec::Vec;
use core::ops::Range;

impl<T> IntoSetGen for Range<T>
where
    usize: From<T>,
    Range<T>: Iterator<Item = T>,
{
    type SetGen<'m> = Vec<usize>;

    fn into_set_gen<'m>(self) -> Self::SetGen<'m> {
        self.map(usize::from).collect()
    }
}
