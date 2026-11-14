use crate::{
    Elements, Res, ResValid, Set, SetCatInt, SetCatStr, SetOrd, SetTyp, Val, ValInt, ValNum, ValTyp,
};
use alloc::boxed::Box;
use core::mem::{transmute, ManuallyDrop};

// core

pub type FP1<'d, V, I0> = Box<dyn Fn(<I0 as Set>::IdxRaw<'d>) -> <V as Val>::Raw + 'd>;

// v

#[repr(C)]
pub union UFunP1V<'d, V: Val> {
    pub cint: ManuallyDrop<FP1<'d, V, SetCatInt>>,
    pub cstr: ManuallyDrop<FP1<'d, V, SetCatStr>>,
    pub ord: ManuallyDrop<FP1<'d, V, SetOrd>>,
}

impl<'d, V: Val> UFunP1V<'d, V> {
    fn empty(i0_typ: SetTyp) -> ManuallyDrop<Self> {
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

    fn create<I0: Set>(fun: FP1<'d, V, I0>) -> ManuallyDrop<Self> {
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

    fn get<I0: Set>(&self) -> &FP1<'d, V, I0> {
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

// v-i0

#[repr(C)]
pub union UFunP1<'d> {
    pub int: ManuallyDrop<UFunP1V<'d, ValInt>>,
    pub num: ManuallyDrop<UFunP1V<'d, ValNum>>,
}

#[derive(derive_new::new)]
pub struct FunP1<'d> {
    pub val_typ: ValTyp,
    pub i0_typ: SetTyp,
    data: UFunP1<'d>,
}

impl<'d> Drop for FunP1<'d> {
    fn drop(&mut self) {
        match self.val_typ {
            ValTyp::Int => unsafe { &mut self.data.int }.man_drop(self.i0_typ),
            ValTyp::Num => unsafe { &mut self.data.num }.man_drop(self.i0_typ),
        }
    }
}

impl<'d> FunP1<'d> {
    pub fn empty(val_typ: ValTyp, i0_typ: SetTyp) -> Self {
        let data = match val_typ {
            ValTyp::Int => UFunP1 {
                int: UFunP1V::empty(i0_typ),
            },
            ValTyp::Num => UFunP1 {
                num: UFunP1V::empty(i0_typ),
            },
        };
        Self::new(val_typ, i0_typ, data)
    }

    pub fn create<V: Val, I0: Set>(fun: FP1<'d, V, I0>) -> Self {
        let (val_typ, i0_typ) = (V::typ(), I0::typ());
        let data = unsafe {
            match val_typ {
                ValTyp::Int => UFunP1 {
                    int: UFunP1V::create::<I0>(transmute(fun)),
                },
                ValTyp::Num => UFunP1 {
                    num: UFunP1V::create::<I0>(transmute(fun)),
                },
            }
        };
        Self::new(val_typ, i0_typ, data)
    }

    pub fn get<V: Val, I0: Set>(&self) -> &FP1<'d, V, I0> {
        Res::dbg_assert_same_val(V::typ(), self.val_typ);
        Res::dbg_assert_same_set(I0::typ(), self.i0_typ);
        unsafe {
            match V::typ() {
                ValTyp::Int => transmute(self.data.int.get::<I0>()),
                ValTyp::Num => transmute(self.data.num.get::<I0>()),
            }
        }
    }
}
