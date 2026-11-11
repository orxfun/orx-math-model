use crate::{idxs::IdxVal, sets::SetType};
use alloc::string::String;

pub trait Idx<'d> {
    fn set_type() -> SetType;

    fn as_idx_val(self) -> IdxVal<'d>;
}

// cat set int

pub struct CatSetInt;

impl<'d> Idx<'d> for usize {
    fn set_type() -> SetType {
        SetType::CatSetInt
    }

    fn as_idx_val(self) -> IdxVal<'d> {
        IdxVal::CatIdxInt(self)
    }
}

// cat set str

pub struct CatSetStr;

impl<'d> Idx<'d> for &'d String {
    fn set_type() -> SetType {
        SetType::CatSetStr
    }

    fn as_idx_val(self) -> IdxVal<'d> {
        IdxVal::CatIdxStr(self)
    }
}

// ord set

pub struct OrdSet;

impl<'d> Idx<'d> for i64 {
    fn set_type() -> SetType {
        SetType::OrdSet
    }

    fn as_idx_val(self) -> IdxVal<'d> {
        IdxVal::OrdIdx(self)
    }
}
