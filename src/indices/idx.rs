use crate::indices::CatIdx;
use crate::ptr::{RefPtr, ValPtr};
use crate::sets::{CatSet, DSet, DSet2, OrdSet};
use crate::values::Int;
use crate::Model;
use orx_imp_vec::Elem;

// data

pub enum DIdx2<'d> {
    CatUsize(RefPtr<DSet2<'d>>, ValPtr<usize>),
    CatStr(RefPtr<DSet2<'d>>, ValPtr<&'d str>),
    Ord(RefPtr<DSet2<'d>>, ValPtr<i64>),
}

impl<'d> DIdx2<'d> {
    pub fn new<'m>(m: &'m Model<'d>, data: Elem<'m, DSet2<'d>>) -> Self {
        let s = RefPtr::from(data);
        match unsafe { data.get() } {
            DSet2::CatUsize(_) => Self::CatUsize(s, ValPtr::from(m.val_usize())),
            DSet2::CatStr(_) => Self::CatStr(s, ValPtr::from(m.val_str())),
            DSet2::Ord(_) => Self::Ord(s, ValPtr::from(m.val_i64())),
        }
    }
}

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

    pub fn set(&self) -> &DSet<'d> {
        self.set.val()
    }

    // TODO: add panics
    pub fn val_ptr_int(&self) -> ValPtr<usize> {
        match &self.val_ptr {
            ValIdxPtr::Int(x) => *x,
            _ => unreachable!(),
        }
    }

    // TODO: add panics
    pub fn val_ptr_str(&self) -> ValPtr<&'d str> {
        match &self.val_ptr {
            ValIdxPtr::Str(x) => *x,
            _ => unreachable!(),
        }
    }
}

// sym

pub enum Idx<'m, 'd> {
    CatInt(CatIdx<'m, 'd, usize>),
    CatStr(CatIdx<'m, 'd, &'d str>),
    Ord(Int<'m, 'd>),
}

impl<'m, 'd> From<CatIdx<'m, 'd, usize>> for Idx<'m, 'd> {
    fn from(value: CatIdx<'m, 'd, usize>) -> Self {
        Self::CatInt(value)
    }
}

impl<'m, 'd> From<CatIdx<'m, 'd, &'d str>> for Idx<'m, 'd> {
    fn from(value: CatIdx<'m, 'd, &'d str>) -> Self {
        Self::CatStr(value)
    }
}

impl<'m, 'd, I: Into<Int<'m, 'd>>> From<I> for Idx<'m, 'd> {
    fn from(value: I) -> Self {
        Self::Ord(value.into())
    }
}
