use core::mem::transmute;

use crate::Model;
use orx_imp_vec::Elem;

#[derive(derive_new::new)]
#[new(visibility = "pub(crate)")]
pub struct Symbol<'m, 'd, T> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, T>,
}

impl<'m, 'd, T> Clone for Symbol<'m, 'd, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'m, 'd, T> Copy for Symbol<'m, 'd, T> {}

impl<'m, 'd, T> Symbol<'m, 'd, T> {
    pub(crate) unsafe fn create_sym_ref(self) -> Symbol<'d, 'd, T> {
        // TODO: this is dangerous
        unsafe { transmute(Self::new(self.m, self.el)) }
    }

    pub(crate) unsafe fn from_sym_ref(sym_ref: Symbol<'d, 'd, T>) -> Self {
        // TODO: this is dangerous
        unsafe { transmute(Self::new(sym_ref.m, sym_ref.el)) }
    }
}
