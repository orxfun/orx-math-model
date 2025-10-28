use crate::symbols::sets::set_gen::{Elements, SetGen};
use alloc::vec::Vec;
use core::ops::Range;

pub struct SetRange<T>
where
    T: Into<usize>,
    Range<T>: Iterator<Item = T> + Clone,
{
    range: Range<T>,
    values: Vec<usize>,
}

impl<T> From<Range<T>> for SetRange<T>
where
    T: Into<usize>,
    Range<T>: Iterator<Item = T> + Clone,
{
    fn from(range: Range<T>) -> Self {
        let values = range.clone().map(|x| x.into()).collect();
        Self { range, values }
    }
}

impl<T> SetGen for SetRange<T>
where
    T: Into<usize>,
    Range<T>: Iterator<Item = T> + Clone,
{
    fn elements<'a>(&'a self, _: usize, _: &'a mut [usize]) -> Elements<'a> {
        Elements::Owned(&self.values)
    }
}
