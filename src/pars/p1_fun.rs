use crate::{Set, SetCatInt, SetCatStr, SetOrd, SetTyp, ValTyp};
use alloc::boxed::Box;
use core::mem::{transmute, ManuallyDrop};

pub type FP1<'d, I0> = Box<dyn Fn(<I0 as Set>::IdxRaw<'d>) -> f64 + 'd>;

// .

#[repr(C)]
pub union UFunP1<'d> {
    pub cint: ManuallyDrop<FP1<'d, SetCatInt>>,
    pub cstr: ManuallyDrop<FP1<'d, SetCatStr>>,
    pub ord: ManuallyDrop<FP1<'d, SetOrd>>,
}

impl<'d> UFunP1<'d> {
    pub fn empty(i0_typ: SetTyp) -> ManuallyDrop<Self> {
        ManuallyDrop::new(match i0_typ {
            SetTyp::CatInt => Self {
                cint: ManuallyDrop::new(Box::new(|_| Default::default())),
            },
            SetTyp::CatStr => Self {
                cstr: ManuallyDrop::new(Box::new(|_| Default::default())),
            },
            SetTyp::Ord => Self {
                ord: ManuallyDrop::new(Box::new(|_| Default::default())),
            },
        })
    }

    pub fn create<I0: Set>(fun: FP1<'d, I0>) -> ManuallyDrop<Self> {
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

    pub fn man_drop(&mut self, i0_typ: SetTyp) {
        unsafe {
            match i0_typ {
                SetTyp::CatInt => ManuallyDrop::drop(&mut self.cint),
                SetTyp::CatStr => ManuallyDrop::drop(&mut self.cstr),
                SetTyp::Ord => ManuallyDrop::drop(&mut self.ord),
            }
        }
    }
}

// fun

pub struct FunP1<'d> {
    val_typ: ValTyp,
    i0_typ: SetTyp,
    data: ManuallyDrop<UFunP1<'d>>,
}

impl<'d> Drop for FunP1<'d> {
    fn drop(&mut self) {
        self.data.man_drop(self.i0_typ);
    }
}

impl<'d> FunP1<'d> {
    pub fn empty(val_typ: ValTyp, i0_typ: SetTyp) -> Self {
        let data = UFunP1::empty(i0_typ);
        Self {
            val_typ,
            i0_typ,
            data,
        }
    }

    pub fn create<I0: Set>(val_typ: ValTyp, fun: FP1<'d, I0>) -> Self {
        let data = UFunP1::create::<I0>(fun);
        Self {
            val_typ,
            i0_typ: I0::typ(),
            data,
        }
    }
}
