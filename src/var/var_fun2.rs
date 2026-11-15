use crate::{Res, ResValid, Set, SetCatInt, SetCatStr, SetOrd, SetTyp};
use alloc::boxed::Box;
use core::mem::{transmute, ManuallyDrop};

// core

type Idx<'d, I0, I1> = (<I0 as Set>::IdxRaw<'d>, <I1 as Set>::IdxRaw<'d>);

pub type FV2<'d, I0, I1> = Box<dyn Fn() -> Idx<'d, I0, I1> + 'd>;

// i0

#[repr(C)]
pub union UFunV2_I0<'d, I0: Set> {
    cint: ManuallyDrop<FV2<'d, I0, SetCatInt>>,
    cstr: ManuallyDrop<FV2<'d, I0, SetCatStr>>,
    ord: ManuallyDrop<FV2<'d, I0, SetOrd>>,
}

impl<'d, I0: Set> UFunV2_I0<'d, I0> {
    fn create<I1: Set>(fun: FV2<'d, I0, I1>) -> ManuallyDrop<Self> {
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

    fn get<I1: Set>(&self) -> &FV2<'d, I0, I1> {
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

// .

#[repr(C)]
pub union UFunV2<'d> {
    cint: ManuallyDrop<UFunV2_I0<'d, SetCatInt>>,
    cstr: ManuallyDrop<UFunV2_I0<'d, SetCatStr>>,
    ord: ManuallyDrop<UFunV2_I0<'d, SetOrd>>,
}

#[derive(derive_new::new)]
pub struct FunV2<'d> {
    pub i0_typ: SetTyp,
    pub i1_typ: SetTyp,
    data: UFunV2<'d>,
}

impl<'d> Drop for FunV2<'d> {
    fn drop(&mut self) {
        match self.i0_typ {
            SetTyp::CatInt => unsafe { &mut self.data.cint }.man_drop(self.i1_typ),
            SetTyp::CatStr => unsafe { &mut self.data.cstr }.man_drop(self.i1_typ),
            SetTyp::Ord => unsafe { &mut self.data.ord }.man_drop(self.i1_typ),
        }
    }
}

impl<'d> FunV2<'d> {
    fn create<I0: Set, I1: Set>(fun: FV2<'d, I0, I1>) -> Self {
        let (i0_typ, i1_typ) = (I0::typ(), I1::typ());
        let data = unsafe {
            match i0_typ {
                SetTyp::CatInt => UFunV2 {
                    cint: transmute(UFunV2_I0::<I0>::create::<I1>(fun)),
                },
                SetTyp::CatStr => UFunV2 {
                    cstr: transmute(UFunV2_I0::<I0>::create::<I1>(fun)),
                },
                SetTyp::Ord => UFunV2 {
                    ord: transmute(UFunV2_I0::<I0>::create::<I1>(fun)),
                },
            }
        };
        Self::new(i0_typ, i1_typ, data)
    }

    fn get<I0: Set, I1: Set>(&self) -> &FV2<'d, I0, I1> {
        Res::dbg_assert_same_set(I0::typ(), self.i0_typ);
        Res::dbg_assert_same_set(I1::typ(), self.i1_typ);
        unsafe {
            match I0::typ() {
                SetTyp::CatInt => transmute(self.data.cint.get::<I1>()),
                SetTyp::CatStr => transmute(self.data.cstr.get::<I1>()),
                SetTyp::Ord => transmute(self.data.ord.get::<I1>()),
            }
        }
    }
}
