use crate::{Set, SetCatInt, SetCatStr, SetOrd, SetTyp};
use alloc::boxed::Box;
use core::mem::{transmute, ManuallyDrop};

// core

type Idx<'d, I0> = <I0 as Set>::IdxRaw<'d>;

pub type FV1<'d, I0> = Box<dyn Fn() -> Idx<'d, I0> + 'd>;

// .

#[repr(C)]
pub union UFunV1<'d> {
    cint: ManuallyDrop<FV1<'d, SetCatInt>>,
    cstr: ManuallyDrop<FV1<'d, SetCatStr>>,
    ord: ManuallyDrop<FV1<'d, SetOrd>>,
}

impl<'d> UFunV1<'d> {
    fn create<I0: Set>(fun: FV1<'d, I0>) -> ManuallyDrop<Self> {
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

    fn get<I0: Set>(&self) -> &FV1<'d, I0> {
        unsafe {
            match I0::typ() {
                SetTyp::CatInt => transmute(&self.cint),
                SetTyp::CatStr => transmute(&self.cstr),
                SetTyp::Ord => transmute(&self.ord),
            }
        }
    }

    fn man_drop(&mut self, i0_typ: SetTyp) {
        unsafe {
            match i0_typ {
                SetTyp::CatInt => ManuallyDrop::drop(&mut self.cint),
                SetTyp::CatStr => ManuallyDrop::drop(&mut self.cstr),
                SetTyp::Ord => ManuallyDrop::drop(&mut self.ord),
            }
        }
    }
}
