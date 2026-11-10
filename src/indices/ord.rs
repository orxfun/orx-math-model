use crate::elements::{Elements, ElementsIter};
use crate::indices::idx::{DIdx, DIdx2};
use crate::Model;
use core::{iter::Copied, slice::Iter};
use orx_imp_vec::Elem;

// symbol

#[derive(derive_new::new, Clone, Copy)]
#[new(visibility = "pub(crate)")]
pub struct OrdIdx2<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, DIdx2<'d>>,
}

#[derive(derive_new::new, Clone, Copy)]
#[new(visibility = "pub(crate)")]
pub struct OrdIdx<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, DIdx<'d>>,
}

impl<'m, 'd> OrdIdx<'m, 'd> {
    pub(crate) fn iter<'a>(
        self,
        elements: &'a mut Elements<usize>,
    ) -> ElementsIter<usize, Copied<Iter<'a, usize>>> {
        let data = unsafe { self.el.get() };
        data.set().as_ord().fill(elements);
        ElementsIter::new(elements.iter(), data.val_ptr_int())
    }
}
