use crate::{idx_scalars::Idx, ptr::RefPtr, sets::elements::Elements, Model};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// fun

#[derive(derive_new::new)]
pub struct SetFun<'d, K>(Box<dyn Fn(&mut Elements<K>) + 'd>);

impl<'d, K> SetFun<'d, K> {
    pub fn empty() -> Self {
        Self(Box::new(|_| {}))
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
#[new(visibility = "pub(crate)")]
pub struct Set<'m, 'd, K> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, SetFun<'d, K>>,
}

impl<'m, 'd, K> Set<'m, 'd, K> {
    pub(crate) fn fun_ptr(&self) -> RefPtr<SetFun<'d, K>> {
        // TODO: safety note
        unsafe { RefPtr::new(self.el.as_ptr()) }
    }
}

impl<'m, 'd> Set<'m, 'd, usize> {
    pub fn idx(self) -> Idx<'m, 'd, usize> {
        self.m.idx_scalar_int(self)
    }
}

impl<'m, 'd> Set<'m, 'd, &'d str> {
    pub fn idx(self) -> Idx<'m, 'd, &'d str> {
        self.m.idx_scalar_str(self)
    }
}
