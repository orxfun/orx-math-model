use crate::ptr::RefPtr;
use crate::symbols::{SetFun, SetOfIdxFun};
use crate::Model;
use orx_imp_vec::Elem;

// data

#[derive(derive_new::new)]
pub struct IdxData<'d, T> {
    s0: RefPtr<SetFun<'d, T>>,
    value: usize,
}

impl<'d, T> IdxData<'d, T> {
    pub fn s0(&self) -> RefPtr<SetFun<'d, T>> {
        self.s0
    }

    pub fn value(&self) -> usize {
        self.value
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct Idx<'m, 'd, T> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, IdxData<'d, T>>,
}

impl<'m, 'd, T> Idx<'m, 'd, T> {
    pub(crate) fn val_ptr(&self) -> *const usize {
        unsafe { core::ptr::addr_of!((*self.el.as_ptr()).value) }
    }
}
