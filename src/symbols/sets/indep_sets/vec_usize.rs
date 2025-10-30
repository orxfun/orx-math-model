use crate::symbols::sets::{elements::Elements, set_gen::SetGen};
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
    fn elements(&self, _: usize, storage: &mut Elements) {
        storage.fill_from_slice(&self.values);
    }
}
