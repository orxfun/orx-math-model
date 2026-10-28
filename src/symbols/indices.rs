use crate::ptr::RefPtr;
use crate::symbols::{Idx, SetFun};
use crate::Model;
use orx_imp_vec::Elem;

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct Indices<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, SetFun<'d>>,
}

impl<'m, 'd> Indices<'m, 'd> {
    pub(crate) fn fun_ptr(&self) -> RefPtr<SetFun<'d>> {
        // TODO: safety note
        unsafe { RefPtr::new(self.el.as_ptr()) }
    }

    pub fn idx(self) -> Idx<'m, 'd> {
        self.m.idx(self)
    }
}
