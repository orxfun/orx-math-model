use crate::sets::FunSet;
use crate::{Ptr, Set, SetCatInt, SetCatStr, SetOrd, SetTyp};
use alloc::boxed::Box;
use core::mem::{transmute, ManuallyDrop};

// core

pub type Idx1<'d, I0> = <I0 as Set>::IdxRaw<'d>;

pub type FVar1<'d, I0> = Box<dyn Fn() -> Idx1<'d, I0> + 'd>;

// .

#[repr(C)]
pub union UFunV1<'d> {
    cint: ManuallyDrop<FVar1<'d, SetCatInt>>,
    cstr: ManuallyDrop<FVar1<'d, SetCatStr>>,
    ord: ManuallyDrop<FVar1<'d, SetOrd>>,
}

#[derive(derive_new::new)]
pub struct FunV1<'d> {
    pub i0: Ptr<'d, FunSet<'d>>,
    data: UFunV1<'d>,
}

impl<'d> Drop for FunV1<'d> {
    fn drop(&mut self) {
        unsafe {
            match self.i0_typ() {
                SetTyp::CatInt => ManuallyDrop::drop(&mut self.data.cint),
                SetTyp::CatStr => ManuallyDrop::drop(&mut self.data.cstr),
                SetTyp::Ord => ManuallyDrop::drop(&mut self.data.ord),
            }
        }
    }
}

impl<'d> FunV1<'d> {
    fn i0_typ(&self) -> SetTyp {
        self.i0.get().typ
    }

    pub fn create<I0: Set>(i0: Ptr<'d, FunSet<'d>>, fun: FVar1<'d, I0>) -> Self {
        debug_assert_eq!(I0::typ(), i0.get().typ);
        let i0_typ = I0::typ();
        let data = unsafe {
            match i0_typ {
                SetTyp::CatInt => UFunV1 {
                    cint: ManuallyDrop::new(transmute(fun)),
                },
                SetTyp::CatStr => UFunV1 {
                    cstr: ManuallyDrop::new(transmute(fun)),
                },
                SetTyp::Ord => UFunV1 {
                    ord: ManuallyDrop::new(transmute(fun)),
                },
            }
        };
        Self::new(i0, data)
    }

    pub fn get<I0: Set>(&self) -> &FVar1<'d, I0> {
        unsafe {
            match self.i0_typ() {
                SetTyp::CatInt => transmute(&self.data.cint),
                SetTyp::CatStr => transmute(&self.data.cstr),
                SetTyp::Ord => transmute(&self.data.ord),
            }
        }
    }
}
