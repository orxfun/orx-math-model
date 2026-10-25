use crate::ptr::RefPtr;
use crate::symbols::{Element, IndicesFun};
use crate::Model;
use orx_imp_vec::Elem;

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct Set<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, IndicesFun<'d>>,
}

impl<'m, 'd> Set<'m, 'd> {
    pub(crate) fn fun_ptr(&self) -> RefPtr<IndicesFun<'d>> {
        // TODO: safety note
        unsafe { RefPtr::new(self.el.as_ptr()) }
    }

    pub fn idx(self) -> Element<'m, 'd> {
        self.m.element(self)
    }
}
