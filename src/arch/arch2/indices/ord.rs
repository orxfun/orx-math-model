use crate::ptr::RefPtr;
use crate::sets::{OrdSet, OrdSetFun};
use crate::Model;
use orx_imp_vec::Elem;

// data

pub struct OrdIdxData<'d> {
    s0: RefPtr<OrdSetFun<'d>>,
    value: usize,
}

impl<'d> OrdIdxData<'d> {
    pub fn new(set: OrdSet<'_, 'd>) -> Self {
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
pub struct OrdIdx<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, OrdIdxData<'d>>,
}
