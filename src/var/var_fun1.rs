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

#[derive(derive_new::new)]
pub struct FunV1<'d> {
    pub i0_typ: SetTyp,
    data: UFunV1<'d>,
}

impl<'d> Drop for FunV1<'d> {
    fn drop(&mut self) {
        unsafe {
            match self.i0_typ {
                SetTyp::CatInt => ManuallyDrop::drop(&mut self.data.cint),
                SetTyp::CatStr => ManuallyDrop::drop(&mut self.data.cstr),
                SetTyp::Ord => ManuallyDrop::drop(&mut self.data.ord),
            }
        }
    }
}

impl<'d> FunV1<'d> {
    fn create<I0: Set>(fun: FV1<'d, I0>) -> Self {
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
        Self::new(i0_typ, data)
    }

    fn get<I0: Set>(&self) -> &FV1<'d, I0> {
        unsafe {
            match self.i0_typ {
                SetTyp::CatInt => transmute(&self.data.cint),
                SetTyp::CatStr => transmute(&self.data.cstr),
                SetTyp::Ord => transmute(&self.data.ord),
            }
        }
    }
}
