use crate::indices::{DIdx, OrdIdx};
use crate::sets::{elements::Elements, DSet};
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// data

#[derive(derive_new::new)]
pub struct DOrdSet<'d> {
    fun: Box<dyn Fn(&mut Elements<usize>) + 'd>,
}

impl<'d> DOrdSet<'d> {
    pub fn empty() -> Self {
        Self {
            fun: Box::new(|_| {}),
        }
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
#[new(visibility = "pub(crate)")]
pub struct OrdSet<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, DSet<'d>>,
}

impl<'m, 'd> OrdSet<'m, 'd> {
    pub fn idx(self) -> OrdIdx<'m, 'd> {
        OrdIdx::new(self.m, self.m.index(DIdx::empty_ord(self)))
    }
}
