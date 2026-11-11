use crate::idxs::IdxVal;
use alloc::string::String;

pub trait MetaSet {
    type Idx<'d>;

    fn idx_as_enum<'d>(idx: Self::Idx<'d>) -> IdxVal<'d>;
}

// cat set int

pub struct MetaCatSetInt;

impl MetaSet for MetaCatSetInt {
    type Idx<'d> = usize;

    fn idx_as_enum<'d>(idx: Self::Idx<'d>) -> IdxVal<'d> {
        IdxVal::CatIdxInt(idx)
    }
}

// cat set str

pub struct MetaCatSetStr;

impl MetaSet for MetaCatSetStr {
    type Idx<'d> = &'d String;

    fn idx_as_enum<'d>(idx: Self::Idx<'d>) -> IdxVal<'d> {
        IdxVal::CatIdxStr(idx)
    }
}

// ord set

pub struct MetaOrdSet;

impl MetaSet for MetaOrdSet {
    type Idx<'d> = i64;

    fn idx_as_enum<'d>(idx: Self::Idx<'d>) -> IdxVal<'d> {
        IdxVal::OrdIdx(idx)
    }
}
