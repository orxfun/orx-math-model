use crate::indices::CatIdx;
use crate::ptr::{RefPtr, ValPtr};
use crate::sets::{CatSet, DSet, OrdSet};

// data

pub enum ValIdxPtr<'d> {
    Int(ValPtr<usize>),
    Str(ValPtr<&'d str>),
}

#[derive(derive_new::new)]
pub struct DIdx<'d> {
    set: RefPtr<DSet<'d>>,
    val_ptr: ValIdxPtr<'d>,
}

impl<'d> DIdx<'d> {
    pub fn empty_cat_int(set: CatSet<'_, 'd, usize>) -> Self {
        let val_ptr = ValIdxPtr::Int(ValPtr::from(set.m.val_usize()));
        let set = RefPtr::from(set.el);
        Self { set, val_ptr }
    }

    pub fn empty_cat_str(set: CatSet<'_, 'd, &'d str>) -> Self {
        let val_ptr = ValIdxPtr::Str(ValPtr::from(set.m.val_str()));
        let set = RefPtr::from(set.el);
        Self { set, val_ptr }
    }

    pub fn empty_ord(set: OrdSet<'_, 'd>) -> Self {
        let val_ptr = ValIdxPtr::Int(ValPtr::from(set.m.val_usize()));
        let set = RefPtr::from(set.el);
        Self { set, val_ptr }
    }
}

// sym

pub enum Idx<'m, 'd> {
    CatInt(CatIdx<'m, 'd, usize>),
    CatStr(CatIdx<'m, 'd, &'d str>),
    Ord(),
}
