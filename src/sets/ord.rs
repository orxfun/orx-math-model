use crate::{indices::OrdIdx, ptr::RefPtr, sets::elements::Elements, Model};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// fun

#[derive(derive_new::new)]
pub struct OrdSetFun<'d>(Box<dyn Fn(&mut Elements<usize>) + 'd>);

impl<'d> OrdSetFun<'d> {
    pub fn empty() -> Self {
        Self(Box::new(|_| {}))
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
#[new(visibility = "pub(crate)")]
pub struct OrdSet<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, OrdSetFun<'d>>,
}

impl<'m, 'd> OrdSet<'m, 'd> {
    pub(crate) fn fun_ptr(&self) -> RefPtr<OrdSetFun<'d>> {
        // TODO: safety note
        unsafe { RefPtr::new(self.el.as_ptr()) }
    }

    pub fn idx(self) -> OrdIdx<'m, 'd> {
        self.m.idx_ord(self)
    }
}
