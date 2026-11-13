use crate::idxs::{DIdx, IdxCatInt, IdxCatStr, IdxOrd, UIdxVal};
// use crate::idxs::{CatIdxInt, CatIdxStr, DIdx, OrdIdx};
use crate::sets::{DSet, FSet, FunSet, SetCatInt, SetCatStr, SetOrd, UFunSet};
use crate::{Model, Ptr, Res, ResValid, Slot, Symbol};
use alloc::string::ToString;
use alloc::{boxed::Box, string::String};
use core::mem::transmute;
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

pub trait MetaSet: Clone + Copy + 'static {
    // set

    type SetSym<'m, 'd: 'm>: Copy + Deref<Target = Symbol<'m, 'd, DSet<'d>>>;

    fn typ() -> SetTyp;

    fn set_unpack<'a, 'd>(u: &'a UFunSet<'d>) -> &'a FSet<'d, Self>;

    fn set_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DSet<'d>>) -> Self::SetSym<'m, 'd>;

    fn set_ptr<'m, 'd>(sym: Self::SetSym<'m, 'd>) -> Ptr<'d, DSet<'d>>;

    // idx

    type IdxRaw<'d>: Copy;

    type IdxSym<'m, 'd: 'm>;

    fn idx_unpack<'a, 'd>(u: &'a UIdxVal<'d>) -> Self::IdxRaw<'d>;

    fn idx_write<'d>(u: &mut UIdxVal<'d>, value: Self::IdxRaw<'d>);

    fn new_idx_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DIdx<'d>>) -> Self::IdxSym<'m, 'd>;

    fn idx_sym<'m, 'd>(sym: Self::IdxSym<'m, 'd>) -> Symbol<'m, 'd, DIdx<'d>>;

    // type IdxSym<'m, 'd>: Idx<'m, 'd, Meta = Self>
    // where
    //     'd: 'm;

    // type IdxSum<'m, 'd>
    // where
    //     'd: 'm;

    // fn idx_unpack_val<'d>(u: UIdxVal<'d>) -> Self::IdxRaw<'d>;

    // fn idx_as_enum<'d>(idx: Self::IdxRaw<'d>) -> IdxVal<'d>;

    // fn create_idx_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DIdx<'d>>) -> Self::IdxSym<'m, 'd>;

    // par-sca

    // fn p1_scalar_fun<'m, 'd: 'm, V: MetaVal>(
    //     m: &'m Model<'d>,
    //     i0: Self::IdxSum<'m, 'd>,
    //     p1: Ptr<'d, Box<dyn Fn(Self::IdxRaw<'d>) -> V::Val + 'd>>,
    // ) -> Box<dyn Fn() -> V::Val + 'd>;
}

// cat set int

#[derive(Clone, Copy)]
pub struct MetaCatInt;

impl MetaSet for MetaCatInt {
    // set

    type SetSym<'m, 'd: 'm> = SetCatInt<'m, 'd>;

    fn typ() -> SetTyp {
        SetTyp::CatInt
    }

    fn set_unpack<'a, 'd>(u: &'a UFunSet<'d>) -> &'a FSet<'d, Self> {
        unsafe { &u.cat_int }
    }

    fn set_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DSet<'d>>) -> Self::SetSym<'m, 'd> {
        Symbol::new(m, el).into()
    }

    fn set_ptr<'m, 'd>(sym: Self::SetSym<'m, 'd>) -> Ptr<'d, DSet<'d>> {
        Ptr::new(sym.el)
    }

    // idx

    type IdxRaw<'d> = usize;

    type IdxSym<'m, 'd: 'm> = IdxCatInt<'m, 'd>;

    fn idx_unpack<'a, 'd>(u: &'a UIdxVal<'d>) -> Self::IdxRaw<'d> {
        unsafe { u.cat_int }
    }

    fn idx_write<'d>(u: &mut UIdxVal<'d>, value: Self::IdxRaw<'d>) {
        u.cat_int = value;
    }

    fn new_idx_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DIdx<'d>>) -> Self::IdxSym<'m, 'd> {
        Symbol::new(m, el).into()
    }

    fn idx_sym<'m, 'd>(sym: Self::IdxSym<'m, 'd>) -> Symbol<'m, 'd, DIdx<'d>> {
        sym.0
    }

    // type IdxSym<'m, 'd>
    //     = CatIdxInt<'m, 'd>
    // where
    //     'd: 'm;

    // type IdxSum<'m, 'd>
    //     = CatIntIdxSum<'m, 'd>
    // where
    //     'd: 'm;

    // fn idx_unpack_val<'d>(u: UIdxVal<'d>) -> Self::IdxRaw<'d> {
    //     unsafe { u.cat_int }
    // }

    // fn idx_as_enum<'d>(idx: Self::IdxRaw<'d>) -> IdxVal<'d> {
    //     IdxVal::CatInt(idx)
    // }

    // fn create_idx_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DIdx<'d>>) -> Self::IdxSym<'m, 'd> {
    //     Symbol::new(m, el).into()
    // }

    // par-sca

    // fn p1_scalar_fun<'m, 'd: 'm, V: MetaVal>(
    //     _: &'m Model<'d>,
    //     i0: Self::IdxSum<'m, 'd>,
    //     p1: Ptr<'d, Box<dyn Fn(Self::IdxRaw<'d>) -> V::Val + 'd>>,
    // ) -> Box<dyn Fn() -> V::Val + 'd> {
    //     match i0 {
    //         CatIntIdxSum::Usize(i0) => Box::new(move || p1.get()(i0)),
    //         CatIntIdxSum::Idx(i0) => {
    //             let i0 = unsafe { i0.el.get() }.idx.val_ptr::<MetaCatInt>();
    //             Box::new(move || p1.get()(i0.val()))
    //         }
    //     }
    // }
}

// cat set str

#[derive(Clone, Copy)]
pub struct MetaCatStr;

impl MetaSet for MetaCatStr {
    // set

    type SetSym<'m, 'd: 'm> = SetCatStr<'m, 'd>;

    fn typ() -> SetTyp {
        SetTyp::CatStr
    }

    fn set_unpack<'a, 'd>(u: &'a UFunSet<'d>) -> &'a FSet<'d, Self> {
        unsafe { &u.cat_str }
    }

    fn set_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DSet<'d>>) -> Self::SetSym<'m, 'd> {
        Symbol::new(m, el).into()
    }

    fn set_ptr<'m, 'd>(sym: Self::SetSym<'m, 'd>) -> Ptr<'d, DSet<'d>> {
        Ptr::new(sym.el)
    }

    // idx

    type IdxRaw<'d> = &'d String;

    type IdxSym<'m, 'd: 'm> = IdxCatStr<'m, 'd>;

    fn idx_unpack<'a, 'd>(u: &'a UIdxVal<'d>) -> Self::IdxRaw<'d> {
        unsafe { u.cat_str }
    }

    fn idx_write<'d>(u: &mut UIdxVal<'d>, value: Self::IdxRaw<'d>) {
        u.cat_str = value;
    }

    fn new_idx_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DIdx<'d>>) -> Self::IdxSym<'m, 'd> {
        Symbol::new(m, el).into()
    }

    fn idx_sym<'m, 'd>(sym: Self::IdxSym<'m, 'd>) -> Symbol<'m, 'd, DIdx<'d>> {
        sym.0
    }

    // type IdxSym<'m, 'd>
    //     = CatIdxStr<'m, 'd>
    // where
    //     'd: 'm;

    // type IdxSum<'m, 'd>
    //     = CatStrIdxSum<'m, 'd>
    // where
    //     'd: 'm;

    // fn idx_unpack_val<'d>(u: UIdxVal<'d>) -> Self::IdxRaw<'d> {
    //     unsafe { u.cat_str }
    // }

    // fn idx_as_enum<'d>(idx: Self::IdxRaw<'d>) -> IdxVal<'d> {
    //     IdxVal::CatStr(idx)
    // }

    // fn create_idx_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DIdx<'d>>) -> Self::IdxSym<'m, 'd> {
    //     Symbol::new(m, el).into()
    // }

    // par-sca

    // fn p1_scalar_fun<'m, 'd: 'm, V: MetaVal>(
    //     m: &'m Model<'d>,
    //     i0: Self::IdxSum<'m, 'd>,
    //     p1: Ptr<'d, Box<dyn Fn(Self::IdxRaw<'d>) -> V::Val + 'd>>,
    // ) -> Box<dyn Fn() -> V::Val + 'd> {
    //     match i0 {
    //         CatStrIdxSum::RefString(i0) => Box::new(move || p1.get()(i0)),
    //         CatStrIdxSum::String(i0) => {
    //             let i0 = unsafe { m.arb_val(i0).get() }.as_string_ptr();
    //             Box::new(move || p1.get()(unsafe { &*i0 }))
    //         }
    //         CatStrIdxSum::Str(i0) => {
    //             let i0 = unsafe { m.arb_val(i0.to_string()).get() }.as_string_ptr();
    //             Box::new(move || p1.get()(unsafe { &*i0 }))
    //         }
    //         CatStrIdxSum::Idx(i0) => {
    //             let i0 = unsafe { i0.el.get() }.idx.val_ptr::<MetaCatStr>();
    //             Box::new(move || p1.get()(i0.val()))
    //         }
    //     }
    // }
}

// ord set

#[derive(Clone, Copy)]
pub struct MetaOrd;

impl MetaSet for MetaOrd {
    // set

    type SetSym<'m, 'd: 'm> = SetOrd<'m, 'd>;

    fn typ() -> SetTyp {
        SetTyp::Ord
    }

    fn set_unpack<'a, 'd>(u: &'a UFunSet<'d>) -> &'a FSet<'d, Self> {
        unsafe { &u.ord }
    }

    fn set_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DSet<'d>>) -> Self::SetSym<'m, 'd> {
        Symbol::new(m, el).into()
    }

    fn set_ptr<'m, 'd>(sym: Self::SetSym<'m, 'd>) -> Ptr<'d, DSet<'d>> {
        Ptr::new(sym.el)
    }

    // idx

    type IdxRaw<'d> = i64;

    type IdxSym<'m, 'd: 'm> = IdxOrd<'m, 'd>;

    fn idx_unpack<'a, 'd>(u: &'a UIdxVal<'d>) -> Self::IdxRaw<'d> {
        unsafe { u.ord }
    }

    fn idx_write<'d>(u: &mut UIdxVal<'d>, value: Self::IdxRaw<'d>) {
        u.ord = value;
    }

    fn new_idx_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DIdx<'d>>) -> Self::IdxSym<'m, 'd> {
        Symbol::new(m, el).into()
    }

    fn idx_sym<'m, 'd>(sym: Self::IdxSym<'m, 'd>) -> Symbol<'m, 'd, DIdx<'d>> {
        sym.0
    }

    // type IdxSym<'m, 'd>
    //     = OrdIdx<'m, 'd>
    // where
    //     'd: 'm;

    // type IdxSum<'m, 'd>
    //     = OrdIdxSum<'m, 'd>
    // where
    //     'd: 'm;

    // fn idx_unpack_val<'d>(u: UIdxVal<'d>) -> Self::IdxRaw<'d> {
    //     unsafe { u.ord }
    // }

    // fn idx_as_enum<'d>(idx: Self::IdxRaw<'d>) -> IdxVal<'d> {
    //     IdxVal::Ord(idx)
    // }

    // fn create_idx_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DIdx<'d>>) -> Self::IdxSym<'m, 'd> {
    //     Symbol::new(m, el).into()
    // }

    // par-sca

    // fn p1_scalar_fun<'m, 'd: 'm, V: MetaVal>(
    //     _: &'m Model<'d>,
    //     i0: Self::IdxSum<'m, 'd>,
    //     p1: Ptr<'d, Box<dyn Fn(Self::IdxRaw<'d>) -> V::Val + 'd>>,
    // ) -> Box<dyn Fn() -> V::Val + 'd> {
    //     match i0 {
    //         OrdIdxSum::Raw(i0) => Box::new(move || p1.get()(i0)),
    //         OrdIdxSum::Idx(i0) => {
    //             let i0 = unsafe { i0.el.get() }.idx.val_ptr::<MetaOrd>();
    //             Box::new(move || p1.get()(i0.val()))
    //         }
    //         OrdIdxSum::Int(i0) => {
    //             let i0 = unsafe { i0.el.get() }.fun_ptr::<MetaInt>() as *const Box<dyn Fn() -> i64>;
    //             Box::new(move || p1.get()(unsafe { &*i0 }()))
    //         }
    //     }
    // }
}

// symbols

// pub trait Set<'m, 'd>: Deref<Target = Symbol<'m, 'd, DSet<'d>>> + Clone + Copy
// where
//     'd: 'm,
// {
//     type Meta: MetaSet;

//     fn ptr(self) -> Ptr<'d, DSet<'d>>;
// }

// impl<'m, 'd> Set<'m, 'd> for SetCatInt<'m, 'd> {
//     type Meta = MetaCatInt;

//     fn ptr(self) -> Ptr<'d, DSet<'d>> {
//         Ptr::new(self.el)
//     }
// }

// impl<'m, 'd> Set<'m, 'd> for SetCatStr<'m, 'd> {
//     type Meta = MetaCatStr;

//     fn ptr(self) -> Ptr<'d, DSet<'d>> {
//         Ptr::new(self.el)
//     }
// }

// impl<'m, 'd> Set<'m, 'd> for SetOrd<'m, 'd> {
//     type Meta = MetaOrd;

//     fn ptr(self) -> Ptr<'d, DSet<'d>> {
//         Ptr::new(self.el)
//     }
// }
