use core::ops::Deref;

use crate::idxs::{CatIdxInt, CatIdxStr, DIdx, OrdIdx};
use crate::par::p1::P1Fun;
use crate::sets::{CatSetInt, CatSetStr, OrdSet, SetSlot};
use crate::{idxs::IdxVal, vals::MetaVal};
use crate::{CatIntIndexer, CatStrIndexer, Idx, Model, OrdIndexer, Symbol};
use alloc::{boxed::Box, string::String};
use orx_imp_vec::Elem;

// enums

pub enum SetTyp {
    CatSetInt,
    CatSetStr,
    OrdSet,
}

// meta

pub trait MetaSet: Clone + Copy {
    // set

    type SetSym<'m, 'd>: Set<'m, 'd, Meta = Self>
    where
        'd: 'm;

    fn set_typ() -> SetTyp;

    fn create_set_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, SetSlot<'d>>) -> Self::SetSym<'m, 'd>;

    // idx

    type IdxRaw<'d>;

    type IdxSym<'m, 'd>: Idx<'m, 'd, Meta = Self>
    where
        'd: 'm;

    type IdxSum<'m, 'd>
    where
        'd: 'm;

    fn idx_as_enum<'d>(idx: Self::IdxRaw<'d>) -> IdxVal<'d>;

    fn idx_sum_into_cat_int<'m, 'd>(idx_sum: Self::IdxSum<'m, 'd>) -> CatIntIndexer<'m, 'd>;

    fn idx_sum_into_cat_str<'m, 'd>(idx_sum: Self::IdxSum<'m, 'd>) -> CatStrIndexer<'m, 'd>;

    fn idx_sum_into_ord<'m, 'd>(idx_sum: Self::IdxSum<'m, 'd>) -> OrdIndexer<'m, 'd>;

    fn create_idx_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DIdx<'d>>) -> Self::IdxSym<'m, 'd>;

    // par

    fn into_p1_fun<'d, V: MetaVal>(fun: impl Fn(Self::IdxRaw<'d>) -> V::Val + 'd) -> P1Fun<'d, V>;
}

// cat set int

#[derive(Clone, Copy)]
pub struct MetaCatSetInt;

impl MetaSet for MetaCatSetInt {
    // set

    type SetSym<'m, 'd>
        = CatSetInt<'m, 'd>
    where
        'd: 'm;

    fn set_typ() -> SetTyp {
        SetTyp::CatSetInt
    }

    fn create_set_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, SetSlot<'d>>) -> Self::SetSym<'m, 'd> {
        Symbol::new(m, el).into()
    }

    // idx

    type IdxRaw<'d> = usize;

    type IdxSym<'m, 'd>
        = CatIdxInt<'m, 'd>
    where
        'd: 'm;

    type IdxSum<'m, 'd>
        = CatIntIndexer<'m, 'd>
    where
        'd: 'm;

    fn idx_as_enum<'d>(idx: Self::IdxRaw<'d>) -> IdxVal<'d> {
        IdxVal::CatIdxInt(idx)
    }

    fn idx_sum_into_cat_int<'m, 'd>(idx_sum: Self::IdxSum<'m, 'd>) -> CatIntIndexer<'m, 'd> {
        idx_sum
    }

    fn idx_sum_into_cat_str<'m, 'd>(_: Self::IdxSum<'m, 'd>) -> CatStrIndexer<'m, 'd> {
        unreachable!()
    }

    fn idx_sum_into_ord<'m, 'd>(_: Self::IdxSum<'m, 'd>) -> OrdIndexer<'m, 'd> {
        unreachable!()
    }

    fn create_idx_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DIdx<'d>>) -> Self::IdxSym<'m, 'd> {
        Symbol::new(m, el).into()
    }

    // par

    fn into_p1_fun<'d, V: MetaVal>(fun: impl Fn(Self::IdxRaw<'d>) -> V::Val + 'd) -> P1Fun<'d, V> {
        P1Fun::CatSetInt(Box::new(fun))
    }
}

// cat set str

#[derive(Clone, Copy)]
pub struct MetaCatSetStr;

impl MetaSet for MetaCatSetStr {
    // set

    type SetSym<'m, 'd>
        = CatSetStr<'m, 'd>
    where
        'd: 'm;

    fn set_typ() -> SetTyp {
        SetTyp::CatSetStr
    }

    fn create_set_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, SetSlot<'d>>) -> Self::SetSym<'m, 'd> {
        Symbol::new(m, el).into()
    }

    // idx

    type IdxRaw<'d> = &'d String;

    type IdxSym<'m, 'd>
        = CatIdxStr<'m, 'd>
    where
        'd: 'm;

    type IdxSum<'m, 'd>
        = CatStrIndexer<'m, 'd>
    where
        'd: 'm;

    fn idx_as_enum<'d>(idx: Self::IdxRaw<'d>) -> IdxVal<'d> {
        IdxVal::CatIdxStr(idx)
    }

    fn idx_sum_into_cat_int<'m, 'd>(_: Self::IdxSum<'m, 'd>) -> CatIntIndexer<'m, 'd> {
        unreachable!()
    }

    fn idx_sum_into_cat_str<'m, 'd>(idx_sum: Self::IdxSum<'m, 'd>) -> CatStrIndexer<'m, 'd> {
        idx_sum
    }

    fn idx_sum_into_ord<'m, 'd>(_: Self::IdxSum<'m, 'd>) -> OrdIndexer<'m, 'd> {
        unreachable!()
    }

    fn create_idx_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DIdx<'d>>) -> Self::IdxSym<'m, 'd> {
        Symbol::new(m, el).into()
    }

    // par

    fn into_p1_fun<'d, V: MetaVal>(fun: impl Fn(Self::IdxRaw<'d>) -> V::Val + 'd) -> P1Fun<'d, V> {
        P1Fun::CatSetStr(Box::new(fun))
    }
}

// ord set

#[derive(Clone, Copy)]
pub struct MetaOrdSet;

impl MetaSet for MetaOrdSet {
    // set

    type SetSym<'m, 'd>
        = OrdSet<'m, 'd>
    where
        'd: 'm;

    fn set_typ() -> SetTyp {
        SetTyp::OrdSet
    }

    fn create_set_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, SetSlot<'d>>) -> Self::SetSym<'m, 'd> {
        Symbol::new(m, el).into()
    }

    // idx

    type IdxRaw<'d> = i64;

    type IdxSym<'m, 'd>
        = OrdIdx<'m, 'd>
    where
        'd: 'm;

    type IdxSum<'m, 'd>
        = OrdIndexer<'m, 'd>
    where
        'd: 'm;

    fn idx_as_enum<'d>(idx: Self::IdxRaw<'d>) -> IdxVal<'d> {
        IdxVal::OrdIdx(idx)
    }

    fn idx_sum_into_cat_int<'m, 'd>(_: Self::IdxSum<'m, 'd>) -> CatIntIndexer<'m, 'd> {
        unreachable!()
    }

    fn idx_sum_into_cat_str<'m, 'd>(_: Self::IdxSum<'m, 'd>) -> CatStrIndexer<'m, 'd> {
        unreachable!()
    }

    fn idx_sum_into_ord<'m, 'd>(idx_sum: Self::IdxSum<'m, 'd>) -> OrdIndexer<'m, 'd> {
        idx_sum
    }

    fn create_idx_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DIdx<'d>>) -> Self::IdxSym<'m, 'd> {
        Symbol::new(m, el).into()
    }

    // par

    fn into_p1_fun<'d, V: MetaVal>(fun: impl Fn(Self::IdxRaw<'d>) -> V::Val + 'd) -> P1Fun<'d, V> {
        P1Fun::OrdSet(Box::new(fun))
    }
}

// symbols

pub trait Set<'m, 'd>: Deref<Target = Symbol<'m, 'd, SetSlot<'d>>> + Clone + Copy
where
    'd: 'm,
{
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
