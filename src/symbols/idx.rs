use crate::ptr::RefPtr;
use crate::symbols::IndicesFun;
use crate::Model;
use orx_imp_vec::Elem;

// data

#[derive(derive_new::new)]
pub struct IdxData<'d> {
    s0: RefPtr<IndicesFun<'d>>,
    value: usize,
}

impl<'d> IdxData<'d> {
    pub fn s0(&self) -> RefPtr<IndicesFun<'d>> {
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
pub struct Idx<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, IdxData<'d>>,
}

impl<'m, 'd> Idx<'m, 'd> {
    pub(crate) fn val_ptr(&self) -> *const usize {
        unsafe { core::ptr::addr_of!((*self.el.as_ptr()).value) }
    }
}
