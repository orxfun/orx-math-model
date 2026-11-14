use crate::idxs::{DIdx, IdxCatInt, IdxCatStr, IdxOrd, UIdxVal};
use crate::pars::p1::ParD1;
use crate::pars::p2::ParD2;
use crate::sets::{
    CatInt, CatStr, DSet, FSet, FunSet, IndexCatInt, IndexCatStr, IndexOrd, Ord, UFunSet,
};
use crate::{Model, Ptr, Res, ResValid, SetSym, Slot, Symbol, Val, ValInt};
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::mem::{transmute, ManuallyDrop};
use core::ops::Deref;
use orx_imp_vec::Elem;

// enums

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SetTyp {
    CatInt,
    CatStr,
    Ord,
}

// meta

pub trait Set: Clone + Copy + 'static {
    // set

    type SetSym<'m, 'd: 'm>: SetSym<'m, 'd, Meta = Self>;

    fn typ() -> SetTyp;

    fn into_fun_union<'d>(fun: FSet<'d, Self>) -> UFunSet<'d>;

    fn fun_from_union<'a, 'd>(u: &'a UFunSet<'d>) -> &'a FSet<'d, Self>;

    fn new_set_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DSet<'d>>) -> Self::SetSym<'m, 'd>;

    fn set_ptr<'m, 'd>(sym: Self::SetSym<'m, 'd>) -> Ptr<'d, DSet<'d>>;

    // idx

    type IdxRaw<'d>: Copy;

    type IdxSym<'m, 'd: 'm>;

    type IdxAny<'m, 'd: 'm>;

    fn idx_unpack<'a, 'd>(u: &'a UIdxVal<'d>) -> Self::IdxRaw<'d>;

    fn idx_write<'d>(u: &mut UIdxVal<'d>, value: Self::IdxRaw<'d>);

    fn new_idx_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DIdx<'d>>) -> Self::IdxSym<'m, 'd>;

    fn into_idx_sym<'m, 'd>(sym: Self::IdxSym<'m, 'd>) -> Symbol<'m, 'd, DIdx<'d>>;

    // p1

    fn p1_at<'m, 'd, V: Val>(
        par: ParD1<'m, 'd, V, Self>,
        idx: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd>;

    // p2

    fn p2_at_i0<'m, 'd, V: Val, I0: Set>(
        par: ParD2<'m, 'd, V, I0, Self>,
        i0: impl Fn() -> I0::IdxRaw<'d> + 'd,
        i1: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd>;

    fn p2_at<'m, 'd, V: Val, I1: Set>(
        par: ParD2<'m, 'd, V, Self, I1>,
        i0: impl Into<Self::IdxAny<'m, 'd>>,
        i1: impl Into<I1::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd>;
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

    fn fun_from_union<'a, 'd>(u: &'a UFunSet<'d>) -> &'a FSet<'d, Self> {
        unsafe { &u.cint }
    }

    fn new_set_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DSet<'d>>) -> Self::SetSym<'m, 'd> {
        Symbol::new(m, el).into()
    }

    fn set_ptr<'m, 'd>(sym: Self::SetSym<'m, 'd>) -> Ptr<'d, DSet<'d>> {
        Ptr::new(sym.el)
    }

    // idx

    type IdxRaw<'d> = usize;

    type IdxSym<'m, 'd: 'm> = IdxCatInt<'m, 'd>;

    type IdxAny<'m, 'd: 'm> = IndexCatInt<'m, 'd>;

    fn idx_unpack<'a, 'd>(u: &'a UIdxVal<'d>) -> Self::IdxRaw<'d> {
        unsafe { u.cat_int }
    }

    fn idx_write<'d>(u: &mut UIdxVal<'d>, value: Self::IdxRaw<'d>) {
        u.cat_int = value;
    }

    fn new_idx_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DIdx<'d>>) -> Self::IdxSym<'m, 'd> {
        Symbol::new(m, el).into()
    }

    fn into_idx_sym<'m, 'd>(sym: Self::IdxSym<'m, 'd>) -> Symbol<'m, 'd, DIdx<'d>> {
        sym.0
    }

    // p1

    fn p1_at<'m, 'd, V: Val>(
        par: ParD1<'m, 'd, V, Self>,
        i0: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd> {
        let fun = Ptr::new(par.el);
        match i0.into() {
            IndexCatInt::Idx(i0) => {
                let i0 = Ptr::new(i0.el);
                par.m.val::<V>(move || {
                    let fun = fun.get();
                    let i0 = i0.get().read::<Self>();
                    fun.at::<V, Self>(i0)
                })
            }
            IndexCatInt::ConUsize(i0) => par.m.val::<V>(move || {
                let fun = fun.get();
                fun.at::<V, Self>(i0)
            }),
        }
    }

    // p2

    fn p2_at_i0<'m, 'd, V: Val, I0: Set>(
        par: ParD2<'m, 'd, V, I0, Self>,
        i0: impl Fn() -> I0::IdxRaw<'d> + 'd,
        i1: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd> {
        let fun = Ptr::new(par.el);
        match i1.into() {
            IndexCatInt::Idx(i1) => {
                let i1 = Ptr::new(i1.el);
                par.m.val::<V>(move || {
                    let fun = fun.get();
                    let i1 = i1.get().read::<Self>();
                    fun.at::<V, I0, Self>(i0(), i1)
                })
            }
            IndexCatInt::ConUsize(i1) => par.m.val::<V>(move || {
                let fun = fun.get();
                fun.at::<V, I0, Self>(i0(), i1)
            }),
        }
    }

    fn p2_at<'m, 'd, V: Val, I1: Set>(
        par: ParD2<'m, 'd, V, Self, I1>,
        i0: impl Into<Self::IdxAny<'m, 'd>>,
        i1: impl Into<I1::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd> {
        match i0.into() {
            IndexCatInt::Idx(i0) => {
                let i0 = Ptr::new(i0.el);
                let i0 = move || i0.get().read::<Self>();
                I1::p2_at_i0(par, i0, i1)
            }
            IndexCatInt::ConUsize(i0) => I1::p2_at_i0(par, move || i0, i1),
        }
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

    fn fun_from_union<'a, 'd>(u: &'a UFunSet<'d>) -> &'a FSet<'d, Self> {
        unsafe { &u.cstr }
    }

    fn new_set_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DSet<'d>>) -> Self::SetSym<'m, 'd> {
        Symbol::new(m, el).into()
    }

    fn set_ptr<'m, 'd>(sym: Self::SetSym<'m, 'd>) -> Ptr<'d, DSet<'d>> {
        Ptr::new(sym.el)
    }

    // idx

    type IdxRaw<'d> = &'d String;

    type IdxSym<'m, 'd: 'm> = IdxCatStr<'m, 'd>;

    type IdxAny<'m, 'd: 'm> = IndexCatStr<'m, 'd>;

    fn idx_unpack<'a, 'd>(u: &'a UIdxVal<'d>) -> Self::IdxRaw<'d> {
        unsafe { u.cat_str }
    }

    fn idx_write<'d>(u: &mut UIdxVal<'d>, value: Self::IdxRaw<'d>) {
        u.cat_str = value;
    }

    fn new_idx_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DIdx<'d>>) -> Self::IdxSym<'m, 'd> {
        Symbol::new(m, el).into()
    }

    fn into_idx_sym<'m, 'd>(sym: Self::IdxSym<'m, 'd>) -> Symbol<'m, 'd, DIdx<'d>> {
        sym.0
    }

    // p1

    fn p1_at<'m, 'd, V: Val>(
        par: ParD1<'m, 'd, V, Self>,
        i0: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd> {
        let fun = Ptr::new(par.el);
        match i0.into() {
            IndexCatStr::Idx(i0) => {
                let i0 = Ptr::new(i0.el);
                par.m.val::<V>(move || {
                    let fun = fun.get();
                    let i0 = i0.get().read::<Self>();
                    fun.at::<V, Self>(i0)
                })
            }
            IndexCatStr::ConString(i0) => {
                let con = par.m.con_val(i0);
                par.m.val::<V>(move || {
                    let fun = fun.get();
                    fun.at::<V, Self>(con.get().get())
                })
            }
            IndexCatStr::ConStringRef(i0) => par.m.val::<V>(move || {
                let fun = fun.get();
                fun.at::<V, Self>(i0)
            }),
            IndexCatStr::ConStr(i0) => {
                let con = par.m.con_val(i0.to_string());
                par.m.val::<V>(move || {
                    let fun = fun.get();
                    fun.at::<V, Self>(con.get().get())
                })
            }
        }
    }

    // p2

    fn p2_at_i0<'m, 'd, V: Val, I0: Set>(
        par: ParD2<'m, 'd, V, I0, Self>,
        i0: impl Fn() -> I0::IdxRaw<'d> + 'd,
        i1: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd> {
        let fun = Ptr::new(par.el);
        todo!()
    }

    fn p2_at<'m, 'd, V: Val, I1: Set>(
        par: ParD2<'m, 'd, V, Self, I1>,
        i0: impl Into<Self::IdxAny<'m, 'd>>,
        i1: impl Into<I1::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd> {
        todo!()
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

    fn fun_from_union<'a, 'd>(u: &'a UFunSet<'d>) -> &'a FSet<'d, Self> {
        unsafe { &u.ord }
    }

    fn new_set_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DSet<'d>>) -> Self::SetSym<'m, 'd> {
        Symbol::new(m, el).into()
    }

    fn set_ptr<'m, 'd>(sym: Self::SetSym<'m, 'd>) -> Ptr<'d, DSet<'d>> {
        Ptr::new(sym.el)
    }

    // idx

    type IdxRaw<'d> = i64;

    type IdxSym<'m, 'd: 'm> = IdxOrd<'m, 'd>;

    type IdxAny<'m, 'd: 'm> = IndexOrd<'m, 'd>;

    fn idx_unpack<'a, 'd>(u: &'a UIdxVal<'d>) -> Self::IdxRaw<'d> {
        unsafe { u.ord }
    }

    fn idx_write<'d>(u: &mut UIdxVal<'d>, value: Self::IdxRaw<'d>) {
        u.ord = value;
    }

    fn new_idx_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DIdx<'d>>) -> Self::IdxSym<'m, 'd> {
        Symbol::new(m, el).into()
    }

    fn into_idx_sym<'m, 'd>(sym: Self::IdxSym<'m, 'd>) -> Symbol<'m, 'd, DIdx<'d>> {
        sym.0
    }

    // p1

    fn p1_at<'m, 'd, V: Val>(
        par: ParD1<'m, 'd, V, Self>,
        i0: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd> {
        let fun = Ptr::new(par.el);
        match i0.into() {
            IndexOrd::Idx(i0) => {
                let i0 = Ptr::new(i0.el);
                par.m.val::<V>(move || {
                    let fun = fun.get();
                    let i0 = i0.get().read::<Self>();
                    fun.at::<V, Self>(i0)
                })
            }
            IndexOrd::Int(i0) => {
                let int = Ptr::new(i0.el);
                par.m.val::<V>(move || {
                    let fun = fun.get();
                    let idx = int.get().eval::<ValInt>();
                    fun.at::<V, Self>(idx)
                })
            }
            IndexOrd::ConUsize(i0) => par.m.val::<V>(move || {
                let fun = fun.get();
                fun.at::<V, Self>(i0 as i64)
            }),
            IndexOrd::ConI64(i0) => par.m.val::<V>(move || {
                let fun = fun.get();
                fun.at::<V, Self>(i0)
            }),
            IndexOrd::ConI32(i0) => par.m.val::<V>(move || {
                let fun = fun.get();
                fun.at::<V, Self>(i0 as i64)
            }),
        }
    }

    // p2

    fn p2_at_i0<'m, 'd, V: Val, I0: Set>(
        par: ParD2<'m, 'd, V, I0, Self>,
        i0: impl Fn() -> I0::IdxRaw<'d> + 'd,
        i1: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd> {
        let fun = Ptr::new(par.el);
        todo!()
    }

    fn p2_at<'m, 'd, V: Val, I1: Set>(
        par: ParD2<'m, 'd, V, Self, I1>,
        i0: impl Into<Self::IdxAny<'m, 'd>>,
        i1: impl Into<I1::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd> {
        todo!()
    }
}
