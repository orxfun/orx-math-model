use crate::idxs::IdxVal;
use alloc::string::String;

pub trait Idx<'d> {
    fn as_idx_val(self) -> IdxVal<'d>;
}

// cat set int

impl<'d> Idx<'d> for usize {
    fn as_idx_val(self) -> IdxVal<'d> {
        IdxVal::CatIdxInt(self)
    }
}

// cat set str

impl<'d> Idx<'d> for &'d String {
    fn as_idx_val(self) -> IdxVal<'d> {
        IdxVal::CatIdxStr(self)
    }
}

// ord set

impl<'d> Idx<'d> for i64 {
    fn as_idx_val(self) -> IdxVal<'d> {
        IdxVal::OrdIdx(self)
    }
}
