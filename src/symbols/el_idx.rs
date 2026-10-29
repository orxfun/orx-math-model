use crate::ptr::RefPtr;
use crate::symbols::SetFun;
use crate::Model;
use orx_imp_vec::Elem;

// data

#[derive(derive_new::new)]
pub struct ElIdxData<'d> {
    s0: RefPtr<SetFun<'d>>,
    value: usize,
}

impl<'d> ElIdxData<'d> {
    pub fn s0(&self) -> RefPtr<SetFun<'d>> {
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
pub struct ElIdx<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, ElIdxData<'d>>,
}

impl<'m, 'd> ElIdx<'m, 'd> {
    pub(crate) fn val_ptr(&self) -> *const usize {
        unsafe { core::ptr::addr_of!((*self.el.as_ptr()).value) }
    }
}
