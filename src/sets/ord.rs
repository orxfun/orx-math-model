use crate::elements::Elements;
use crate::indices::{DIdx2, OrdIdx2};
use crate::sets::DSet2;
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// symbol

#[derive(derive_new::new, Clone, Copy)]
#[new(visibility = "pub(crate)")]
pub struct OrdSet2<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, DSet2<'d>>,
}

impl<'m, 'd> OrdSet2<'m, 'd> {
    pub fn idx(self) -> OrdIdx2<'m, 'd> {
        let data = DIdx2::new(self.m, self.el);
        OrdIdx2::new(self.m, self.m.index(data))
    }
}
