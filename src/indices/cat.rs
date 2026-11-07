use crate::elements::{Elements, ElementsIter};
use crate::indices::idx::DIdx;
use crate::Model;
use core::marker::PhantomData;
use core::{iter::Copied, slice::Iter};
use orx_imp_vec::Elem;

#[derive(derive_new::new, Clone, Copy)]
#[new(visibility = "pub(crate)")]
pub struct CatIdx<'m, 'd, K: Copy + Default> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, DIdx<'d>>,
    p: PhantomData<K>,
}

impl<'m, 'd> CatIdx<'m, 'd, usize> {
    pub(crate) fn iter<'a>(
        self,
        elements: &'a mut Elements<usize>,
    ) -> ElementsIter<usize, Copied<Iter<'a, usize>>> {
        let data = unsafe { self.el.get() };
        data.set().as_cat_int().fill(elements);
        ElementsIter::new(elements.iter(), data.val_ptr_int())
    }
}

impl<'m, 'd> CatIdx<'m, 'd, &'d str> {
    pub(crate) fn iter<'a>(
        self,
        elements: &'a mut Elements<&'d str>,
    ) -> ElementsIter<&'d str, Copied<Iter<'a, &'d str>>> {
        let data = unsafe { self.el.get() };
        data.set().as_cat_str().fill(elements);
        ElementsIter::new(elements.iter(), data.val_ptr_str())
    }
}
