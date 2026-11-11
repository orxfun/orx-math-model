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
