use crate::symbols::RefSetOfIdx;
use crate::Model;
use orx_imp_vec::Elem;

// data

#[derive(derive_new::new)]
pub struct IdxData {
    s0: RefSetOfIdx,
    value: usize,
}

impl IdxData {
    pub fn s0(&self) -> RefSetOfIdx {
        self.s0
    }

    pub fn value(&self) -> usize {
        self.value
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
    }

    pub fn ptr_value(p: *const IdxData) -> *const usize {
        unsafe { core::ptr::addr_of!((*p).value) }
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct Idx<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, IdxData>,
}

impl<'m, 'd> Idx<'m, 'd> {
    pub(crate) fn val_ptr(&self) -> *const usize {
        unsafe { core::ptr::addr_of!((*self.el.as_ptr()).value) }
    }
}
