use crate::symbols::sets::{elements::Elements, set_gen::SetGen};
use alloc::vec::Vec;
use core::ops::Range;

pub struct SetRange<T>
where
    T: Into<usize>,
    Range<T>: ExactSizeIterator<Item = T> + Clone,
{
    range: Range<T>,
    values: Vec<usize>,
}

impl<T> From<Range<T>> for SetRange<T>
where
    T: Into<usize>,
    Range<T>: ExactSizeIterator<Item = T> + Clone,
{
    fn from(range: Range<T>) -> Self {
        let values = range.clone().map(|x| x.into()).collect();
        Self { range, values }
    }
}

impl<T> SetGen for SetRange<T>
where
    T: Into<usize>,
    Range<T>: ExactSizeIterator<Item = T> + Clone,
{
    fn elements(&self, _: usize, storage: &mut Elements) {
        storage.fill(
            Some(self.range.len()),
            self.range.clone().into_iter().map(|x| x.into()),
        );
    }
}
