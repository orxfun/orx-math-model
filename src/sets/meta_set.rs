use crate::idxs::{CatIdxInt, CatIdxStr, OrdIdx};
use crate::par::p1::P1Fun;
use crate::sets::{CatSetInt, CatSetStr, OrdSet, SetSlot};
use crate::{idxs::IdxVal, vals::MetaVal};
use alloc::{boxed::Box, string::String};

// enums

pub enum SetTyp {
    CatSetInt,
    CatSetStr,
    OrdSet,
}

// meta

pub trait MetaSet {
    type Idx<'d>;

    type SetSym<'m, 'd>
    where
        'd: 'm;

    type IdxSym<'m, 'd>
    where
        'd: 'm;

    // set

    fn set_typ() -> SetTyp;

    // idx

    fn idx_as_enum<'d>(idx: Self::Idx<'d>) -> IdxVal<'d>;

    // par

    fn into_p1_fun<'d, V: MetaVal>(fun: impl Fn(Self::Idx<'d>) -> V::Val + 'd) -> P1Fun<'d, V>;
}

// cat set int

pub struct MetaCatSetInt;

impl MetaSet for MetaCatSetInt {
    type Idx<'d> = usize;

    type SetSym<'m, 'd>
        = CatSetInt<'m, 'd>
    where
        'd: 'm;

    type IdxSym<'m, 'd>
        = CatIdxInt<'m, 'd>
    where
        'd: 'm;

    // set

    fn set_typ() -> SetTyp {
        SetTyp::CatSetInt
    }

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

    type SetSym<'m, 'd>
        = CatSetStr<'m, 'd>
    where
        'd: 'm;

    type IdxSym<'m, 'd>
        = CatIdxStr<'m, 'd>
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

    type SetSym<'m, 'd>
        = OrdSet<'m, 'd>
    where
        'd: 'm;

    type IdxSym<'m, 'd>
        = OrdIdx<'m, 'd>
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

// symbols

pub trait Set<'m, 'd> {
    type Meta: MetaSet;

    fn slot(self) -> SetSlot<'d>;
}

impl<'m, 'd> Set<'m, 'd> for CatSetInt<'m, 'd> {
    type Meta = MetaCatSetInt;

    fn slot(self) -> SetSlot<'d> {
        unsafe { self.el.get() }.clone()
    }
}

impl<'m, 'd> Set<'m, 'd> for CatSetStr<'m, 'd> {
    type Meta = MetaCatSetStr;

    fn slot(self) -> SetSlot<'d> {
        unsafe { self.el.get() }.clone()
    }
}

impl<'m, 'd> Set<'m, 'd> for OrdSet<'m, 'd> {
    type Meta = MetaOrdSet;

    fn slot(self) -> SetSlot<'d> {
        unsafe { self.el.get() }.clone()
    }
}
