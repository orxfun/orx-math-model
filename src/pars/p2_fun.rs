use crate::{Set, SetCatInt, SetCatStr, SetOrd, SetTyp, ValTyp};
use alloc::boxed::Box;
use core::mem::{transmute, ManuallyDrop};

pub type FP2<'d, I0, I1> =
    Box<dyn Fn(<I0 as Set>::IdxRaw<'d>, <I1 as Set>::IdxRaw<'d>) -> f64 + 'd>;

// i0

#[repr(C)]
pub union UFunP2_I0<'d, I0: Set> {
    pub cint: ManuallyDrop<FP2<'d, I0, SetCatInt>>,
    pub cstr: ManuallyDrop<FP2<'d, I0, SetCatStr>>,
    pub ord: ManuallyDrop<FP2<'d, I0, SetOrd>>,
}

impl<'d, I0: Set> UFunP2_I0<'d, I0> {
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

    fn create<I1: Set>(fun: FP2<'d, I0, I1>) -> ManuallyDrop<Self> {
        unsafe {
            ManuallyDrop::new(match I1::typ() {
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
