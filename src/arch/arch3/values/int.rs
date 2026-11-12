use crate::elements::Elements;
use crate::indices::{DIdx, OrdIdx};
use crate::sets::DSet;
use crate::values::data::DVal;
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// symbol

#[derive(derive_new::new, Clone, Copy)]
#[new(visibility = "pub(crate)")]
pub struct Int<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, DVal<'d>>,
}
