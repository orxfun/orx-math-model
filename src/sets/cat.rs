use crate::elements::Elements;
use crate::indices::{CatIdx, DIdx};
use crate::sets::DSet;
use crate::Model;
use alloc::boxed::Box;
use core::marker::PhantomData;
use orx_imp_vec::Elem;

// symbol

#[derive(derive_new::new, Clone, Copy)]
#[new(visibility = "pub(crate)")]
pub struct CatSet2<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, DSet<'d>>,
}

impl<'m, 'd> CatSet2<'m, 'd> {
    pub fn idx(self) -> CatIdx<'m, 'd> {
        let data = DIdx::new(self.m, self.el);
        CatIdx::new(self.m, self.m.index(data))
    }
}
