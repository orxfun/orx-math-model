use crate::symbols::sets::set_gen::{Elements, SetGen};
use alloc::vec::Vec;

pub struct SetVecUsize {
    values: Vec<usize>,
}

impl From<Vec<usize>> for SetVecUsize {
    fn from(values: Vec<usize>) -> Self {
        Self { values }
    }
}

impl From<&[usize]> for SetVecUsize {
    fn from(values: &[usize]) -> Self {
        let values = values.iter().map(|x| *x as usize).collect();
        Self { values }
    }
}

impl SetGen for SetVecUsize {
    fn elements<'a>(&'a self, _: usize, _: &'a mut [usize]) -> Elements<'a> {
        Elements::Owned(&self.values)
    }
}
