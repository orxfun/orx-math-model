use crate::ptr::{RefPtr, ValPtr};
use crate::sets::DSet;
use crate::values::Int;
use crate::Model;
use orx_imp_vec::Elem;

// data

pub enum DIdx<'d> {
    CatUsize(RefPtr<DSet<'d>>, ValPtr<usize>),
    CatStr(RefPtr<DSet<'d>>, ValPtr<&'d str>),
    Ord(RefPtr<DSet<'d>>, ValPtr<i64>),
}

impl<'d> DIdx<'d> {
    pub fn new<'m>(m: &'m Model<'d>, data: Elem<'m, DSet<'d>>) -> Self {
        let s = RefPtr::from(data);
        match unsafe { data.get() } {
            DSet::CatUsize(_, _) => Self::CatUsize(s, ValPtr::from(m.val_usize())),
            DSet::CatStr(_, _) => Self::CatStr(s, ValPtr::from(m.val_str())),
            DSet::Ord(_, _) => Self::Ord(s, ValPtr::from(m.val_i64())),
        }
    }
}
