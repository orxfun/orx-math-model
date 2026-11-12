use crate::idxs::idx_bag::DIdx;
use crate::idxs::IdxSlot;
use crate::sets::SetSlot;
use crate::{Elements, ElementsIter, Symbol};
use alloc::{boxed::Box, string::String};

// cat int

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct CatIdxInt<'m, 'd>(pub(crate) Symbol<'m, 'd, DIdx<'d>>);

impl<'m, 'd> CatIdxInt<'m, 'd> {
    pub(crate) fn iter<'a>(self, elements: &'a mut Elements<usize>) -> ElementsIter<'a, usize> {
        let data = unsafe { self.el.get() };
        match (&data.set, &data.idx) {
            (SetSlot::CatSetInt(set), IdxSlot::CatIdxInt(idx)) => {
                let set_fun = set.get();
                set_fun(elements);
                ElementsIter::new(elements.iter(), idx.mut_ptr())
            }
            _ => unreachable!("TODO: explain"),
        }
    }
}

// cat str

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct CatIdxStr<'m, 'd>(pub(crate) Symbol<'m, 'd, DIdx<'d>>);

impl<'m, 'd> CatIdxStr<'m, 'd> {
    pub(crate) fn iter<'a>(
        self,
        elements: &'a mut Elements<&'d String>,
    ) -> ElementsIter<'a, &'d String> {
        let data = unsafe { self.el.get() };
        match (&data.set, &data.idx) {
            (SetSlot::CatSetStr(set), IdxSlot::CatIdxStr(idx)) => {
                let set_fun = set.get();
                set_fun(elements);
                ElementsIter::new(elements.iter(), idx.mut_ptr())
            }
            _ => unreachable!("TODO: explain"),
        }
    }
}

// ord

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct OrdIdx<'m, 'd>(pub(crate) Symbol<'m, 'd, DIdx<'d>>);

impl<'m, 'd> OrdIdx<'m, 'd> {
    pub(crate) fn iter<'a>(self, elements: &'a mut Elements<i64>) -> ElementsIter<'a, i64> {
        let data = unsafe { self.el.get() };
        match (&data.set, &data.idx) {
            (SetSlot::OrdSet(set), IdxSlot::OrdIdx(idx)) => {
                let set_fun = set.get();
                set_fun(elements);
                ElementsIter::new(elements.iter(), idx.mut_ptr())
            }
            _ => unreachable!("TODO: explain"),
        }
    }
}
