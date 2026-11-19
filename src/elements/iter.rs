use crate::idxs::{DIdx, IdxVal};
use crate::{Elements, PtrMut, Set};
use core::{iter::Copied, slice::Iter};

// TODO: safety notes, T must be copy and no-drop

pub struct ElementsIter<'a, 'd, S: Set> {
    elements_iter: Copied<Iter<'a, S::IdxRaw<'d>>>,
    idx: PtrMut<'d, IdxVal<'d>>,
}

impl<'a, 'd, S: Set> Iterator for ElementsIter<'a, 'd, S> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        self.elements_iter.next().map(|x| {
            self.idx.get_mut().write::<S>(x);
        })
    }
}

impl<'a, 'd, S: Set> ElementsIter<'a, 'd, S> {
    pub fn new<'m>(idx: S::IdxSym<'m, 'd>, elements: &'a mut Elements<S::IdxRaw<'d>>) -> Self {
        let set = S::idx_sym_to_set_data(idx);
        S::fill_elements(set.get(), elements);
        let sym = S::into_idx_sym(idx);
        let idx = unsafe { sym.el.get() }.val_ptr_mut();
        let elements_iter = elements.iter();
        Self { elements_iter, idx }
    }
}
