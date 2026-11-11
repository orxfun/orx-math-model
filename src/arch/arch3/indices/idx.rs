use crate::elements::{Elements, ElementsIter};
use crate::index_types::{IdxIn, IdxInVariant};
use crate::ptr::{RefPtr, ValPtr};
use crate::sets::DSet;
use crate::Model;
use alloc::string::String;
use orx_imp_vec::Elem;

// data

pub enum DIdx<'d> {
    CatUsize(RefPtr<DSet<'d>>, ValPtr<usize>),
    CatStr(RefPtr<DSet<'d>>, ValPtr<&'d String>),
    Ord(RefPtr<DSet<'d>>, ValPtr<i64>),
}

impl<'d> DIdx<'d> {
    pub fn new<'m>(m: &'m Model<'d>, data: Elem<'m, DSet<'d>>) -> Self {
        let s = RefPtr::from(data);
        match unsafe { data.get() } {
            DSet::CatUsize(_, _) => Self::CatUsize(s, ValPtr::from(m.slot_usize())),
            DSet::CatStr(_, _) => Self::CatStr(s, ValPtr::from(m.slot_str())),
            DSet::Ord(_, _) => Self::Ord(s, ValPtr::from(m.slot_i64())),
        }
    }

    pub fn set(&self) -> &DSet<'d> {
        match self {
            Self::CatUsize(p, _) => p.val(),
            Self::CatStr(p, _) => p.val(),
            Self::Ord(p, _) => p.val(),
        }
    }

    pub fn iter<'a, 'm, I: IdxIn<'d>>(&self, elements: &'a mut Elements<I>) -> ElementsIter<'a, I> {
        I::idx_iter(self, elements)
    }

    pub fn value_as_variant(&self) -> IdxInVariant<'d> {
        match self {
            Self::CatUsize(_, v) => IdxInVariant::Usize(v.val()),
            Self::CatStr(_, v) => IdxInVariant::Str(v.val()),
            Self::Ord(_, v) => IdxInVariant::I64(v.val()),
        }
    }
}
