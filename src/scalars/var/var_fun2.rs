use crate::{sets::FunSet, Ptr, Set, SetCatInt, SetCatStr, SetOrd, SetTyp};
use alloc::boxed::Box;
use core::mem::{transmute, ManuallyDrop};

// core

pub type Idx2<'d, I0, I1> = (<I0 as Set>::IdxRaw<'d>, <I1 as Set>::IdxRaw<'d>);

pub type FVar2<'d, I0, I1> = Box<dyn Fn() -> Idx2<'d, I0, I1> + 'd>;

// i0

#[repr(C)]
pub union UFunV2_I0<'d, I0: Set> {
    pub cint: ManuallyDrop<FVar2<'d, I0, SetCatInt>>,
    pub cstr: ManuallyDrop<FVar2<'d, I0, SetCatStr>>,
    pub ord: ManuallyDrop<FVar2<'d, I0, SetOrd>>,
}

impl<'d, I0: Set> UFunV2_I0<'d, I0> {
    fn create<I1: Set>(fun: FVar2<'d, I0, I1>) -> ManuallyDrop<Self> {
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

    fn get<I1: Set>(&self) -> &FVar2<'d, I0, I1> {
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
    pub cint: ManuallyDrop<UFunV2_I0<'d, SetCatInt>>,
    pub cstr: ManuallyDrop<UFunV2_I0<'d, SetCatStr>>,
    pub ord: ManuallyDrop<UFunV2_I0<'d, SetOrd>>,
}

#[derive(derive_new::new)]
pub struct FunVar2<'d> {
    pub i0: Ptr<'d, FunSet<'d>>,
    pub i1: Ptr<'d, FunSet<'d>>,
    data: UFunV2<'d>,
}

impl<'d> Drop for FunVar2<'d> {
    fn drop(&mut self) {
        let i1_typ = self.i1_typ();
        match self.i0_typ() {
            SetTyp::CatInt => unsafe { &mut self.data.cint }.man_drop(i1_typ),
            SetTyp::CatStr => unsafe { &mut self.data.cstr }.man_drop(i1_typ),
            SetTyp::Ord => unsafe { &mut self.data.ord }.man_drop(i1_typ),
        }
    }
}

impl<'d> FunVar2<'d> {
    fn i0_typ(&self) -> SetTyp {
        self.i0.get().typ
    }

    fn i1_typ(&self) -> SetTyp {
        self.i1.get().typ
    }

    pub fn create<I0: Set, I1: Set>(
        i0: Ptr<'d, FunSet<'d>>,
        i1: Ptr<'d, FunSet<'d>>,
        fun: FVar2<'d, I0, I1>,
    ) -> Self {
        debug_assert_eq!(I0::typ(), i0.get().typ);
        debug_assert_eq!(I1::typ(), i1.get().typ);
        let data = unsafe {
            match I0::typ() {
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
        Self::new(i0, i1, data)
    }

    pub fn get<I0: Set, I1: Set>(&self) -> &FVar2<'d, I0, I1> {
        debug_assert_eq!(I0::typ(), self.i0_typ());
        debug_assert_eq!(I1::typ(), self.i1_typ());
        unsafe {
            match I0::typ() {
                SetTyp::CatInt => transmute(self.data.cint.get::<I1>()),
                SetTyp::CatStr => transmute(self.data.cstr.get::<I1>()),
                SetTyp::Ord => transmute(self.data.ord.get::<I1>()),
            }
        }
    }
}
