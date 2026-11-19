use crate::idxs::{DIdx, IdxCatInt, IdxCatStr, IdxOrd, UIdxVal};
use crate::pars::{IntParD1, IntParD2, NumParD1, NumParD2, UFunP1, UFunP2, UFunP2_I0, FP1, FP2};
use crate::scalars::{DNum, Int, Num, Val, ValInt, ValNum, Var};
use crate::sets::into_idx::IdxAny;
use crate::sets::{CatInt, CatStr, DSet, FSet, IndexCatInt, IndexCatStr, IndexOrd, Ord, UFunSet};
use crate::vars::{VarD1, VarD2};
use crate::{Model, Ptr, SetSym, SymWrap, Symbol, ValTyp};
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::fmt::Write;
use core::mem::{transmute, ManuallyDrop};
use orx_imp_vec::Elem;

// idx types

pub type Idx1<'d, I0> = <I0 as Set>::IdxRaw<'d>;
pub type Idx2<'d, I0, I1> = (<I0 as Set>::IdxRaw<'d>, <I1 as Set>::IdxRaw<'d>);

// enums

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SetTyp {
    CatInt,
    CatStr,
    Ord,
}

pub enum EIdx<'d> {
    CatInt(usize),
    CatStr(&'d String),
    Ord(i64),
}

// meta

pub trait Set: Clone + Copy + 'static {
    // set

    type SetSym<'m, 'd: 'm>: SetSym<'m, 'd, Meta = Self>;

    fn typ() -> SetTyp;

    fn into_fun_union<'d>(fun: FSet<'d, Self>) -> UFunSet<'d>;

    fn unpack_set_fun<'a, 'd>(u: &'a UFunSet<'d>) -> &'a FSet<'d, Self>;

    fn new_set_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DSet<'d>>) -> Self::SetSym<'m, 'd>;

    fn set_ptr<'m, 'd>(sym: Self::SetSym<'m, 'd>) -> Ptr<'d, DSet<'d>>;

    // idx

    type IdxRaw<'d>: Copy;

    type IdxSym<'m, 'd: 'm>;

    type IdxAny<'m, 'd: 'm>: IdxAny<'m, 'd, Set = Self>;

    fn idx_unpack<'a, 'd>(u: &'a UIdxVal<'d>) -> Self::IdxRaw<'d>;

    fn idx_write<'d>(u: &mut UIdxVal<'d>, value: Self::IdxRaw<'d>);

    fn new_idx_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DIdx<'d>>) -> Self::IdxSym<'m, 'd>;

    fn into_idx_sym<'m, 'd>(sym: Self::IdxSym<'m, 'd>) -> Symbol<'m, 'd, DIdx<'d>>;

    fn into_idx_enum<'d>(raw: Self::IdxRaw<'d>) -> EIdx<'d>;

    fn write_idx_str<'d>(f: &mut String, raw: Self::IdxRaw<'d>);

    // p1

    fn unpack_p1_fun<'a, 'd>(u: &'a UFunP1<'d>) -> &'a FP1<'d, Self>;

    fn num_p1_at<'m, 'd>(
        par: NumParD1<'m, 'd, Self>,
        i0: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> Num<'m, 'd> {
        i0.into().p1_at::<ValNum>(par.m(), par.data().fun_ptr())
    }

    fn int_p1_at<'m, 'd>(
        par: IntParD1<'m, 'd, Self>,
        i0: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> Int<'m, 'd> {
        i0.into().p1_at::<ValInt>(par.m(), par.data().fun_ptr())
    }

    // p2

    fn unpack_p2_fun_v_i0<'a, 'd, I0: Set>(u: &'a UFunP2_I0<'d, I0>) -> &'a FP2<'d, I0, Self>;

    fn unpack_p2_fun_v<'a, 'd, I1: Set>(u: &'a UFunP2<'d>) -> &'a FP2<'d, Self, I1>;

    fn num_p2_at<'m, 'd, I1: Set>(
        par: NumParD2<'m, 'd, Self, I1>,
        i0: impl Into<Self::IdxAny<'m, 'd>>,
        i1: impl Into<I1::IdxAny<'m, 'd>>,
    ) -> Num<'m, 'd> {
        i0.into()
            .p2_at::<ValNum, I1::IdxAny<'m, 'd>>(par.m(), par.data().fun_ptr(), i1.into())
    }

    fn int_p2_at<'m, 'd, I1: Set>(
        par: IntParD2<'m, 'd, Self, I1>,
        i0: impl Into<Self::IdxAny<'m, 'd>>,
        i1: impl Into<I1::IdxAny<'m, 'd>>,
    ) -> Int<'m, 'd> {
        i0.into()
            .p2_at::<ValInt, I1::IdxAny<'m, 'd>>(par.m(), par.data().fun_ptr(), i1.into())
    }

    // v1

    fn v1_at<'m, 'd>(var: VarD1<'m, 'd, Self>, i0: impl Into<Self::IdxAny<'m, 'd>>) -> Var<'m, 'd> {
        i0.into().v1_at(var)
    }

    // v2

    fn v2_at<'m, 'd, I1: Set>(
        var: VarD2<'m, 'd, Self, I1>,
        i0: impl Into<Self::IdxAny<'m, 'd>>,
        i1: impl Into<I1::IdxAny<'m, 'd>>,
    ) -> Var<'m, 'd> {
        i0.into().v2_at::<I1>(var, i1.into())
    }
}

// cat set int

#[derive(Clone, Copy)]
pub struct SetCatInt;

impl Set for SetCatInt {
    // set

    type SetSym<'m, 'd: 'm> = CatInt<'m, 'd>;

    fn typ() -> SetTyp {
        SetTyp::CatInt
    }

    fn into_fun_union<'d>(fun: FSet<'d, Self>) -> UFunSet<'d> {
        UFunSet {
            cint: ManuallyDrop::new(fun),
        }
    }

    #[inline(always)]
    fn unpack_set_fun<'a, 'd>(u: &'a UFunSet<'d>) -> &'a FSet<'d, Self> {
        unsafe { &u.cint }
    }

    fn new_set_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DSet<'d>>) -> Self::SetSym<'m, 'd> {
        Self::SetSym::new(Symbol::new(m, el))
    }

    fn set_ptr<'m, 'd>(sym: Self::SetSym<'m, 'd>) -> Ptr<'d, DSet<'d>> {
        sym.ptr()
    }

    // idx

    type IdxRaw<'d> = usize;

    type IdxSym<'m, 'd: 'm> = IdxCatInt<'m, 'd>;

    type IdxAny<'m, 'd: 'm> = IndexCatInt<'m, 'd>;

    #[inline(always)]
    fn idx_unpack<'a, 'd>(u: &'a UIdxVal<'d>) -> Self::IdxRaw<'d> {
        unsafe { u.cat_int }
    }

    #[inline(always)]
    fn idx_write<'d>(u: &mut UIdxVal<'d>, value: Self::IdxRaw<'d>) {
        u.cat_int = value;
    }

    fn new_idx_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DIdx<'d>>) -> Self::IdxSym<'m, 'd> {
        Self::IdxSym::new(Symbol::new(m, el))
    }

    fn into_idx_sym<'m, 'd>(sym: Self::IdxSym<'m, 'd>) -> Symbol<'m, 'd, DIdx<'d>> {
        sym.sym()
    }

    #[inline(always)]
    fn into_idx_enum<'d>(raw: Self::IdxRaw<'d>) -> EIdx<'d> {
        EIdx::CatInt(raw)
    }

    #[inline(always)]
    fn write_idx_str<'d>(f: &mut String, raw: Self::IdxRaw<'d>) {
        write!(f, "{raw}").unwrap()
    }

    // p1

    #[inline(always)]
    fn unpack_p1_fun<'a, 'd>(u: &'a UFunP1<'d>) -> &'a FP1<'d, Self> {
        unsafe { &u.cint }
    }

    // p2

    #[inline(always)]
    fn unpack_p2_fun_v_i0<'a, 'd, I0: Set>(u: &'a UFunP2_I0<'d, I0>) -> &'a FP2<'d, I0, Self> {
        unsafe { &u.cint }
    }

    #[inline(always)]
    fn unpack_p2_fun_v<'a, 'd, I1: Set>(u: &'a UFunP2<'d>) -> &'a FP2<'d, Self, I1> {
        I1::unpack_p2_fun_v_i0(unsafe { &u.cint })
    }
}

// cat set str

#[derive(Clone, Copy)]
pub struct SetCatStr;

impl Set for SetCatStr {
    // set

    type SetSym<'m, 'd: 'm> = CatStr<'m, 'd>;

    fn typ() -> SetTyp {
        SetTyp::CatStr
    }

    fn into_fun_union<'d>(fun: FSet<'d, Self>) -> UFunSet<'d> {
        UFunSet {
            cstr: ManuallyDrop::new(fun),
        }
    }

    #[inline(always)]
    fn unpack_set_fun<'a, 'd>(u: &'a UFunSet<'d>) -> &'a FSet<'d, Self> {
        unsafe { &u.cstr }
    }

    fn new_set_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DSet<'d>>) -> Self::SetSym<'m, 'd> {
        Self::SetSym::new(Symbol::new(m, el))
    }

    fn set_ptr<'m, 'd>(sym: Self::SetSym<'m, 'd>) -> Ptr<'d, DSet<'d>> {
        sym.ptr()
    }

    // idx

    type IdxRaw<'d> = &'d String;

    type IdxSym<'m, 'd: 'm> = IdxCatStr<'m, 'd>;

    type IdxAny<'m, 'd: 'm> = IndexCatStr<'m, 'd>;

    #[inline(always)]
    fn idx_unpack<'a, 'd>(u: &'a UIdxVal<'d>) -> Self::IdxRaw<'d> {
        unsafe { u.cat_str }
    }

    #[inline(always)]
    fn idx_write<'d>(u: &mut UIdxVal<'d>, value: Self::IdxRaw<'d>) {
        u.cat_str = value;
    }

    fn new_idx_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DIdx<'d>>) -> Self::IdxSym<'m, 'd> {
        Self::IdxSym::new(Symbol::new(m, el))
    }

    fn into_idx_sym<'m, 'd>(sym: Self::IdxSym<'m, 'd>) -> Symbol<'m, 'd, DIdx<'d>> {
        sym.sym()
    }

    #[inline(always)]
    fn into_idx_enum<'d>(raw: Self::IdxRaw<'d>) -> EIdx<'d> {
        EIdx::CatStr(raw)
    }

    #[inline(always)]
    fn write_idx_str<'d>(f: &mut String, raw: Self::IdxRaw<'d>) {
        write!(f, "{raw}").unwrap()
    }

    // p1

    #[inline(always)]
    fn unpack_p1_fun<'a, 'd>(u: &'a UFunP1<'d>) -> &'a FP1<'d, Self> {
        unsafe { &u.cstr }
    }

    // p2

    #[inline(always)]
    fn unpack_p2_fun_v_i0<'a, 'd, I0: Set>(u: &'a UFunP2_I0<'d, I0>) -> &'a FP2<'d, I0, Self> {
        unsafe { &u.cstr }
    }

    #[inline(always)]
    fn unpack_p2_fun_v<'a, 'd, I1: Set>(u: &'a UFunP2<'d>) -> &'a FP2<'d, Self, I1> {
        I1::unpack_p2_fun_v_i0(unsafe { &u.cstr })
    }
}

// ord set

#[derive(Clone, Copy)]
pub struct SetOrd;

impl Set for SetOrd {
    // set

    type SetSym<'m, 'd: 'm> = Ord<'m, 'd>;

    fn typ() -> SetTyp {
        SetTyp::Ord
    }

    fn into_fun_union<'d>(fun: FSet<'d, Self>) -> UFunSet<'d> {
        UFunSet {
            ord: ManuallyDrop::new(fun),
        }
    }

    #[inline(always)]
    fn unpack_set_fun<'a, 'd>(u: &'a UFunSet<'d>) -> &'a FSet<'d, Self> {
        unsafe { &u.ord }
    }

    fn new_set_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DSet<'d>>) -> Self::SetSym<'m, 'd> {
        Self::SetSym::new(Symbol::new(m, el))
    }

    fn set_ptr<'m, 'd>(sym: Self::SetSym<'m, 'd>) -> Ptr<'d, DSet<'d>> {
        sym.ptr()
    }

    // idx

    type IdxRaw<'d> = i64;

    type IdxSym<'m, 'd: 'm> = IdxOrd<'m, 'd>;

    type IdxAny<'m, 'd: 'm> = IndexOrd<'m, 'd>;

    #[inline(always)]
    fn idx_unpack<'a, 'd>(u: &'a UIdxVal<'d>) -> Self::IdxRaw<'d> {
        unsafe { u.ord }
    }

    #[inline(always)]
    fn idx_write<'d>(u: &mut UIdxVal<'d>, value: Self::IdxRaw<'d>) {
        u.ord = value;
    }

    fn new_idx_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DIdx<'d>>) -> Self::IdxSym<'m, 'd> {
        Self::IdxSym::new(Symbol::new(m, el))
    }

    fn into_idx_sym<'m, 'd>(sym: Self::IdxSym<'m, 'd>) -> Symbol<'m, 'd, DIdx<'d>> {
        sym.sym()
    }

    #[inline(always)]
    fn into_idx_enum<'d>(raw: Self::IdxRaw<'d>) -> EIdx<'d> {
        EIdx::Ord(raw)
    }

    #[inline(always)]
    fn write_idx_str<'d>(f: &mut String, raw: Self::IdxRaw<'d>) {
        write!(f, "{raw}").unwrap()
    }

    // p1

    #[inline(always)]
    fn unpack_p1_fun<'a, 'd>(u: &'a UFunP1<'d>) -> &'a FP1<'d, Self> {
        unsafe { &u.ord }
    }

    // p2

    #[inline(always)]
    fn unpack_p2_fun_v_i0<'a, 'd, I0: Set>(u: &'a UFunP2_I0<'d, I0>) -> &'a FP2<'d, I0, Self> {
        unsafe { &u.ord }
    }

    #[inline(always)]
    fn unpack_p2_fun_v<'a, 'd, I1: Set>(u: &'a UFunP2<'d>) -> &'a FP2<'d, Self, I1> {
        I1::unpack_p2_fun_v_i0(unsafe { &u.ord })
    }
}
