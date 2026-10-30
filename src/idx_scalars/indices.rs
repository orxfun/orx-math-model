use crate::ptr::RefPtr;
use crate::sets::{Indices, IndicesFun};
use crate::Model;
use orx_imp_vec::Elem;

// data

pub struct IndicesIdxData<'d> {
    s0: RefPtr<IndicesFun<'d>>,
    value: usize,
}

impl<'d> IndicesIdxData<'d> {
    pub fn new(set: Indices<'_, 'd>) -> Self {
        let s0 = set.fun_ptr();
        let value = 0;
        Self { s0, value }
    }

    pub fn read(&self) -> usize {
        self.value
    }

    pub fn write(&mut self, value: usize) {
        self.value = value;
    }
}

// symbol

#[derive(derive_new::new)]
#[new(visibility = "pub(crate)")]
pub struct IndicesIdx<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, IndicesIdxData<'d>>,
}
