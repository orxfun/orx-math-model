use crate::ptr::{RefPtr, ValPtr};
use crate::sets::DSet2;
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
