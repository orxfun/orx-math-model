use crate::indices::idx::DIdx;
use crate::Model;
use orx_imp_vec::Elem;

#[derive(derive_new::new, Clone, Copy)]
#[new(visibility = "pub(crate)")]
pub struct OrdIdx<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, DIdx<'d>>,
}

impl<'m, 'd> OrdIdx<'m, 'd> {
    pub fn iter(self) {
        todo!()
    }
}
