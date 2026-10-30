use crate::{indices::CatIdx, ptr::RefPtr, sets::elements::Elements, Model};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// fun

#[derive(derive_new::new)]
pub struct CatSetFun<'d, K>(Box<dyn Fn(&mut Elements<K>) + 'd>);

impl<'d, K> CatSetFun<'d, K> {
    pub fn empty() -> Self {
        Self(Box::new(|_| {}))
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
#[new(visibility = "pub(crate)")]
pub struct CatSet<'m, 'd, K> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, CatSetFun<'d, K>>,
}

impl<'m, 'd, K> CatSet<'m, 'd, K> {
    pub(crate) fn fun_ptr(&self) -> RefPtr<CatSetFun<'d, K>> {
        // TODO: safety note
        unsafe { RefPtr::new(self.el.as_ptr()) }
    }
}

impl<'m, 'd> CatSet<'m, 'd, usize> {
    pub fn idx(self) -> CatIdx<'m, 'd, usize> {
        self.m.idx_cat_int(self)
    }
}

impl<'m, 'd> CatSet<'m, 'd, &'d str> {
    pub fn idx(self) -> CatIdx<'m, 'd, &'d str> {
        self.m.idx_cat_str(self)
    }
}
