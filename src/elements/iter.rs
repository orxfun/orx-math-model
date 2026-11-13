use crate::{idxs::DIdx, Elements, Set};
use core::{iter::Copied, slice::Iter};

// TODO: safety notes, T must be copy and no-drop

pub struct ElementsIter<'a, 'd, S: Set> {
    elements_iter: Copied<Iter<'a, S::IdxRaw<'d>>>,
    idx: *mut DIdx<'d>,
}

impl<'a, 'd, S: Set> Iterator for ElementsIter<'a, 'd, S> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        self.elements_iter.next().map(|x| {
            unsafe { &mut *self.idx }.write::<S>(x);
        })
    }
}

impl<'a, 'd, S: Set> ElementsIter<'a, 'd, S> {
    pub fn new<'m>(idx: S::IdxSym<'m, 'd>, elements: &'a mut Elements<S::IdxRaw<'d>>) -> Self {
        let sym = S::into_idx_sym(idx);
        let idx = sym.el.as_mut_ptr();
        let elements_iter = elements.iter();
        Self { elements_iter, idx }
    }
}
