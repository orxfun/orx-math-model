use crate::idxs::{CatIdxInt, CatIdxStr, DIdx, OrdIdx};
use crate::par::p1::P1FunE;
use crate::sets::{CatSetInt, CatSetStr, OrdSet, SetSlot};
use crate::{idxs::IdxVal, vals::MetaVal};
use crate::{CatIntIdxSum, CatStrIdxSum, Idx, MetaInt, Model, OrdIdxSum, Ptr, Symbol};
use alloc::string::ToString;
use alloc::{boxed::Box, string::String};
use core::mem::transmute;
use core::ops::Deref;
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

    fn create_idx_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DIdx<'d>>) -> Self::IdxSym<'m, 'd>;

    // par-sca

    fn p1_scalar_fun<'m, 'd: 'm, V: MetaVal>(
        m: &'m Model<'d>,
        i0: Self::IdxSum<'m, 'd>,
        p1: Ptr<'d, Box<dyn Fn(Self::IdxRaw<'d>) -> V::Val + 'd>>,
    ) -> Box<dyn Fn() -> V::Val + 'd>;
}

// cat set int

#[derive(Clone, Copy)]
pub struct MetaCatInt;

impl MetaSet for MetaCatInt {
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
        = CatIntIdxSum<'m, 'd>
    where
        'd: 'm;

    fn idx_as_enum<'d>(idx: Self::IdxRaw<'d>) -> IdxVal<'d> {
        IdxVal::CatIdxInt(idx)
    }

    fn create_idx_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DIdx<'d>>) -> Self::IdxSym<'m, 'd> {
        Symbol::new(m, el).into()
    }

    // par-sca

    fn p1_scalar_fun<'m, 'd: 'm, V: MetaVal>(
        _: &'m Model<'d>,
        i0: Self::IdxSum<'m, 'd>,
        p1: Ptr<'d, Box<dyn Fn(Self::IdxRaw<'d>) -> V::Val + 'd>>,
    ) -> Box<dyn Fn() -> V::Val + 'd> {
        match i0 {
            CatIntIdxSum::Usize(i0) => Box::new(move || p1.get()(i0)),
            CatIntIdxSum::Idx(i0) => {
                let i0 = unsafe { i0.el.get() }.idx.val_ptr::<MetaCatInt>();
                Box::new(move || p1.get()(i0.val()))
            }
        }
    }
}

// cat set str

#[derive(Clone, Copy)]
pub struct MetaCatStr;

impl MetaSet for MetaCatStr {
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
        = CatStrIdxSum<'m, 'd>
    where
        'd: 'm;

    fn idx_as_enum<'d>(idx: Self::IdxRaw<'d>) -> IdxVal<'d> {
        IdxVal::CatIdxStr(idx)
    }

    fn create_idx_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DIdx<'d>>) -> Self::IdxSym<'m, 'd> {
        Symbol::new(m, el).into()
    }

    // par-sca

    fn p1_scalar_fun<'m, 'd: 'm, V: MetaVal>(
        m: &'m Model<'d>,
        i0: Self::IdxSum<'m, 'd>,
        p1: Ptr<'d, Box<dyn Fn(Self::IdxRaw<'d>) -> V::Val + 'd>>,
    ) -> Box<dyn Fn() -> V::Val + 'd> {
        match i0 {
            CatStrIdxSum::RefString(i0) => Box::new(move || p1.get()(i0)),
            CatStrIdxSum::String(i0) => {
                let i0 = unsafe { m.arb_val(i0).get() }.as_string_ptr();
                Box::new(move || p1.get()(unsafe { &*i0 }))
            }
            CatStrIdxSum::Str(i0) => {
                let i0 = unsafe { m.arb_val(i0.to_string()).get() }.as_string_ptr();
                Box::new(move || p1.get()(unsafe { &*i0 }))
            }
            CatStrIdxSum::Idx(i0) => {
                let i0 = unsafe { i0.el.get() }.idx.val_ptr::<MetaCatStr>();
                Box::new(move || p1.get()(i0.val()))
            }
        }
    }
}

// ord set

#[derive(Clone, Copy)]
pub struct MetaOrd;

impl MetaSet for MetaOrd {
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
        = OrdIdxSum<'m, 'd>
    where
        'd: 'm;

    fn idx_as_enum<'d>(idx: Self::IdxRaw<'d>) -> IdxVal<'d> {
        IdxVal::OrdIdx(idx)
    }

    fn create_idx_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DIdx<'d>>) -> Self::IdxSym<'m, 'd> {
        Symbol::new(m, el).into()
    }

    // par-sca

    fn p1_scalar_fun<'m, 'd: 'm, V: MetaVal>(
        _: &'m Model<'d>,
        i0: Self::IdxSum<'m, 'd>,
        p1: Ptr<'d, Box<dyn Fn(Self::IdxRaw<'d>) -> V::Val + 'd>>,
    ) -> Box<dyn Fn() -> V::Val + 'd> {
        match i0 {
            OrdIdxSum::Raw(i0) => Box::new(move || p1.get()(i0)),
            OrdIdxSum::Idx(i0) => {
                let i0 = unsafe { i0.el.get() }.idx.val_ptr::<MetaOrd>();
                Box::new(move || p1.get()(i0.val()))
            }
            OrdIdxSum::Int(i0) => {
                let i0 = unsafe { i0.el.get() }.fun_ptr::<MetaInt>() as *const Box<dyn Fn() -> i64>;
                Box::new(move || p1.get()(unsafe { &*i0 }()))
            }
        }
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
    type Meta = MetaCatInt;

    fn slot(self) -> SetSlot<'d> {
        unsafe { self.el.get() }.clone()
    }
}

impl<'m, 'd> Set<'m, 'd> for CatSetStr<'m, 'd> {
    type Meta = MetaCatStr;

    fn slot(self) -> SetSlot<'d> {
        unsafe { self.el.get() }.clone()
    }
}

impl<'m, 'd> Set<'m, 'd> for OrdSet<'m, 'd> {
    type Meta = MetaOrd;

    fn slot(self) -> SetSlot<'d> {
        unsafe { self.el.get() }.clone()
    }
}
