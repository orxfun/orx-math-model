use crate::{idxs::IdxVal, par::p1::P1Fun, vals::MetaVal};
use alloc::{boxed::Box, string::String};

pub trait MetaSet {
    type Idx<'d>;

    fn idx_as_enum<'d>(idx: Self::Idx<'d>) -> IdxVal<'d>;

    // par

    fn into_p1_fun<'d, V: MetaVal>(fun: impl Fn(Self::Idx<'d>) -> V::Val + 'd) -> P1Fun<'d, V>;
}

// cat set int

pub struct MetaCatSetInt;

impl MetaSet for MetaCatSetInt {
    type Idx<'d> = usize;

    fn idx_as_enum<'d>(idx: Self::Idx<'d>) -> IdxVal<'d> {
        IdxVal::CatIdxInt(idx)
    }

    // par

    fn into_p1_fun<'d, V: MetaVal>(fun: impl Fn(Self::Idx<'d>) -> V::Val + 'd) -> P1Fun<'d, V> {
        P1Fun::CatSetInt(Box::new(fun))
    }
}

// cat set str

pub struct MetaCatSetStr;

impl MetaSet for MetaCatSetStr {
    type Idx<'d> = &'d String;

    fn idx_as_enum<'d>(idx: Self::Idx<'d>) -> IdxVal<'d> {
        IdxVal::CatIdxStr(idx)
    }

    // par

    fn into_p1_fun<'d, V: MetaVal>(fun: impl Fn(Self::Idx<'d>) -> V::Val + 'd) -> P1Fun<'d, V> {
        P1Fun::CatSetStr(Box::new(fun))
    }
}

// ord set

pub struct MetaOrdSet;

impl MetaSet for MetaOrdSet {
    type Idx<'d> = i64;

    fn idx_as_enum<'d>(idx: Self::Idx<'d>) -> IdxVal<'d> {
        IdxVal::OrdIdx(idx)
    }

    // par

    fn into_p1_fun<'d, V: MetaVal>(fun: impl Fn(Self::Idx<'d>) -> V::Val + 'd) -> P1Fun<'d, V> {
        P1Fun::OrdSet(Box::new(fun))
    }
}
