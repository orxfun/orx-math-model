use crate::model_references::SetRef;
use crate::Model;
use orx_imp_vec::Elem;

#[derive(derive_new::new)]
pub struct IdxData {
    s0: SetRef,
    value_ptr: *mut usize,
}

impl IdxData {
    pub fn s0(&self) -> SetRef {
        self.s0
    }

    pub fn read_value(&self) -> usize {
        unsafe { *self.value_ptr }
    }

    pub fn set_value(&mut self, value: usize) {
        unsafe { self.value_ptr.write(value) };
    }

    pub fn value_ptr(&self) -> *mut usize {
        self.value_ptr
    }
}

#[derive(derive_new::new, Clone, Copy)]
pub struct Idx<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, IdxData>,
}
