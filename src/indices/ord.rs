use crate::elements::{Elements, ElementsIter};
use crate::indices::idx::DIdx;
use crate::Model;
use core::{iter::Copied, slice::Iter};
use orx_imp_vec::Elem;

// symbol

#[derive(derive_new::new, Clone, Copy)]
#[new(visibility = "pub(crate)")]
pub struct OrdIdx<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, DIdx<'d>>,
}
