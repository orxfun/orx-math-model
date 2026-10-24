use crate::ptr::RefPtr;
use crate::symbols::SetOfIdxFun;
use crate::Model;
use orx_imp_vec::Elem;

// data

#[derive(derive_new::new)]
pub struct IdxDataDepr<'d> {
    s0: RefPtr<SetOfIdxFun<'d>>,
    value: usize,
}

impl<'d> IdxDataDepr<'d> {
    pub fn s0(&self) -> RefPtr<SetOfIdxFun<'d>> {
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
pub struct IdxDepr<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, IdxDataDepr<'d>>,
}

impl<'m, 'd> IdxDepr<'m, 'd> {
    pub(crate) fn val_ptr(&self) -> *const usize {
        unsafe { core::ptr::addr_of!((*self.el.as_ptr()).value) }
    }
}
