use crate::{Set, SetCatInt, SetCatStr, SetOrd, SetTyp, Val, ValInt, ValNum};
use alloc::boxed::Box;
use core::mem::{transmute, ManuallyDrop};

// core

pub type FP2<'d, V, I0, I1> =
    Box<dyn Fn(<I0 as Set>::IdxRaw<'d>, <I1 as Set>::IdxRaw<'d>) -> <V as Val>::Raw + 'd>;

// v-i0

#[repr(C)]
pub union UFunP2_V_I0<'d, V: Val, I0: Set> {
    pub cint: ManuallyDrop<FP2<'d, V, I0, SetCatInt>>,
    pub cstr: ManuallyDrop<FP2<'d, V, I0, SetCatStr>>,
    pub ord: ManuallyDrop<FP2<'d, V, I0, SetOrd>>,
}

impl<'d, V: Val, I0: Set> UFunP2_V_I0<'d, V, I0> {
    fn empty(i1_typ: SetTyp) -> ManuallyDrop<Self> {
        ManuallyDrop::new(match i1_typ {
            SetTyp::CatInt => Self {
                cint: ManuallyDrop::new(Box::new(|_, _| Default::default())),
            },
            SetTyp::CatStr => Self {
                cstr: ManuallyDrop::new(Box::new(|_, _| Default::default())),
            },
            SetTyp::Ord => Self {
                ord: ManuallyDrop::new(Box::new(|_, _| Default::default())),
            },
        })
    }

    fn create<I1: Set>(fun: FP2<'d, V, I0, I1>) -> ManuallyDrop<Self> {
        unsafe {
            ManuallyDrop::new(match I0::typ() {
                SetTyp::CatInt => Self {
                    cint: ManuallyDrop::new(transmute(fun)),
                },
                SetTyp::CatStr => Self {
                    cstr: ManuallyDrop::new(transmute(fun)),
                },
                SetTyp::Ord => Self {
                    ord: ManuallyDrop::new(transmute(fun)),
                },
            })
        }
    }

    fn get<I1: Set>(&self) -> &FP2<'d, V, I0, I1> {
        unsafe {
            match I0::typ() {
                SetTyp::CatInt => transmute(&self.cint),
                SetTyp::CatStr => transmute(&self.cstr),
                SetTyp::Ord => transmute(&self.ord),
            }
        }
    }

    fn man_drop(&mut self, i1_typ: SetTyp) {
        unsafe {
            match i1_typ {
                SetTyp::CatInt => ManuallyDrop::drop(&mut self.cint),
                SetTyp::CatStr => ManuallyDrop::drop(&mut self.cstr),
                SetTyp::Ord => ManuallyDrop::drop(&mut self.ord),
            }
        }
    }
}

// #[repr(C)]
// pub union UFunP2<'d> {
//     pub int_cint_cint: ManuallyDrop<FP2<'d, ValInt, SetCatInt, SetCatInt>>,
//     pub int_cint_cstr: ManuallyDrop<FP2<'d, ValInt, SetCatInt, SetCatStr>>,
//     pub int_cint_ord: ManuallyDrop<FP2<'d, ValInt, SetCatInt, SetOrd>>,

//     pub int_cstr_cint: ManuallyDrop<FP2<'d, ValInt, SetCatStr, SetCatInt>>,
//     pub int_cstr_cstr: ManuallyDrop<FP2<'d, ValInt, SetCatStr, SetCatStr>>,
//     pub int_cstr_ord: ManuallyDrop<FP2<'d, ValInt, SetCatStr, SetOrd>>,

//     pub int_ord_cint: ManuallyDrop<FP2<'d, ValInt, SetOrd, SetCatInt>>,
//     pub int_ord_cstr: ManuallyDrop<FP2<'d, ValInt, SetOrd, SetCatStr>>,
//     pub int_ord_ord: ManuallyDrop<FP2<'d, ValInt, SetOrd, SetOrd>>,

//     pub num_cint_cint: ManuallyDrop<FP2<'d, ValNum, SetCatInt, SetCatInt>>,
//     pub num_cint_cstr: ManuallyDrop<FP2<'d, ValNum, SetCatInt, SetCatStr>>,
//     pub num_cint_ord: ManuallyDrop<FP2<'d, ValNum, SetCatInt, SetOrd>>,

//     pub num_cstr_cint: ManuallyDrop<FP2<'d, ValNum, SetCatStr, SetCatInt>>,
//     pub num_cstr_cstr: ManuallyDrop<FP2<'d, ValNum, SetCatStr, SetCatStr>>,
//     pub num_cstr_ord: ManuallyDrop<FP2<'d, ValNum, SetCatStr, SetOrd>>,

//     pub num_ord_cint: ManuallyDrop<FP2<'d, ValNum, SetOrd, SetCatInt>>,
//     pub num_ord_cstr: ManuallyDrop<FP2<'d, ValNum, SetOrd, SetCatStr>>,
//     pub num_ord_ord: ManuallyDrop<FP2<'d, ValNum, SetOrd, SetOrd>>,
// }
