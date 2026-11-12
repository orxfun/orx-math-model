use crate::par::p1::P1Fun;
use crate::sets::{CatSetInt, CatSetStr, OrdSet, SetSlot};
use crate::{idxs::IdxVal, vals::MetaVal};
use alloc::{boxed::Box, string::String};

pub enum SetTyp {
    CatSetInt,
    CatSetStr,
    OrdSet,
}

pub trait MetaSet {
    type Idx<'d>;

    type Set<'m, 'd>
    where
        'd: 'm;

    // set

    fn set_typ() -> SetTyp;

    // fn set_slot<'d>(set: Self::Set<'_, 'd>) -> SetSlot<'d>;

    // idx

    fn idx_as_enum<'d>(idx: Self::Idx<'d>) -> IdxVal<'d>;

    // par

    fn into_p1_fun<'d, V: MetaVal>(fun: impl Fn(Self::Idx<'d>) -> V::Val + 'd) -> P1Fun<'d, V>;
}

// cat set int

pub struct MetaCatSetInt;

impl MetaSet for MetaCatSetInt {
    type Idx<'d> = usize;

    type Set<'m, 'd>
        = CatSetInt<'m, 'd>
    where
        'd: 'm;
    // set

    fn set_typ() -> SetTyp {
        SetTyp::CatSetInt
    }

    // fn set_slot<'d>(set: Self::Set<'_, 'd>) -> SetSlot<'d> {
    //     unsafe { set.el.get() }.clone()
    // }

    // idx

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

    type Set<'m, 'd>
        = CatSetStr<'m, 'd>
    where
        'd: 'm;
    // set

    fn set_typ() -> SetTyp {
        SetTyp::CatSetStr
    }

    // idx

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

    type Set<'m, 'd>
        = OrdSet<'m, 'd>
    where
        'd: 'm;
    // set

    fn set_typ() -> SetTyp {
        SetTyp::OrdSet
    }

    // idx

    fn idx_as_enum<'d>(idx: Self::Idx<'d>) -> IdxVal<'d> {
        IdxVal::OrdIdx(idx)
    }

    // par

    fn into_p1_fun<'d, V: MetaVal>(fun: impl Fn(Self::Idx<'d>) -> V::Val + 'd) -> P1Fun<'d, V> {
        P1Fun::OrdSet(Box::new(fun))
    }
}
