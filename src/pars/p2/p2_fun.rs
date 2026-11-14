use crate::{Set, SetCatInt, SetCatStr, SetOrd, SetTyp, Val, ValInt, ValNum, ValTyp};
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
            match I1::typ() {
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

// v

#[repr(C)]
pub union UFunP2_V<'d, V: Val> {
    pub cint: ManuallyDrop<UFunP2_V_I0<'d, V, SetCatInt>>,
    pub cstr: ManuallyDrop<UFunP2_V_I0<'d, V, SetCatStr>>,
    pub ord: ManuallyDrop<UFunP2_V_I0<'d, V, SetOrd>>,
}

impl<'d, V: Val> UFunP2_V<'d, V> {
    fn empty(i0_typ: SetTyp, i1_typ: SetTyp) -> ManuallyDrop<Self> {
        ManuallyDrop::new(match i0_typ {
            SetTyp::CatInt => Self {
                cint: UFunP2_V_I0::empty(i1_typ),
            },
            SetTyp::CatStr => Self {
                cstr: UFunP2_V_I0::empty(i1_typ),
            },
            SetTyp::Ord => Self {
                ord: UFunP2_V_I0::empty(i1_typ),
            },
        })
    }

    fn create<I0: Set, I1: Set>(fun: FP2<'d, V, I0, I1>) -> ManuallyDrop<Self> {
        unsafe {
            ManuallyDrop::new(match I0::typ() {
                SetTyp::CatInt => Self {
                    cint: transmute(UFunP2_V_I0::<V, I0>::create::<I1>(fun)),
                },
                SetTyp::CatStr => Self {
                    cstr: transmute(UFunP2_V_I0::<V, I0>::create::<I1>(fun)),
                },
                SetTyp::Ord => Self {
                    ord: transmute(UFunP2_V_I0::<V, I0>::create::<I1>(fun)),
                },
            })
        }
    }

    fn get<I0: Set, I1: Set>(&self) -> &FP2<'d, V, I0, I1> {
        unsafe {
            match I0::typ() {
                SetTyp::CatInt => transmute(&self.cint.get::<I1>()),
                SetTyp::CatStr => transmute(&self.cstr.get::<I1>()),
                SetTyp::Ord => transmute(&self.ord.get::<I1>()),
            }
        }
    }

    fn man_drop(&mut self, i0_typ: SetTyp, i1_typ: SetTyp) {
        unsafe {
            match i0_typ {
                SetTyp::CatInt => self.cint.man_drop(i1_typ),
                SetTyp::CatStr => self.cstr.man_drop(i1_typ),
                SetTyp::Ord => self.ord.man_drop(i1_typ),
            }
        }
    }
}

// .

#[repr(C)]
pub union UFunP2<'d> {
    pub int: ManuallyDrop<UFunP2_V<'d, ValInt>>,
    pub num: ManuallyDrop<UFunP2_V<'d, ValNum>>,
}

#[derive(derive_new::new)]
pub struct FunP2<'d> {
    pub val_typ: ValTyp,
    pub i0_typ: SetTyp,
    pub i1_typ: SetTyp,
    data: UFunP2<'d>,
}

impl<'d> Drop for FunP2<'d> {
    fn drop(&mut self) {
        match self.val_typ {
            ValTyp::Int => unsafe { &mut self.data.int }.man_drop(self.i0_typ, self.i1_typ),
            ValTyp::Num => unsafe { &mut self.data.num }.man_drop(self.i0_typ, self.i1_typ),
        }
    }
}

impl<'d> FunP2<'d> {
    pub fn empty(val_typ: ValTyp, i0_typ: SetTyp, i1_typ: SetTyp) -> Self {
        let data = match val_typ {
            ValTyp::Int => UFunP2 {
                int: UFunP2_V::empty(i0_typ, i1_typ),
            },
            ValTyp::Num => UFunP2 {
                num: UFunP2_V::empty(i0_typ, i1_typ),
            },
        };
        Self::new(val_typ, i0_typ, i1_typ, data)
    }

    pub fn create<V: Val, I0: Set, I1: Set>(fun: FP2<'d, V, I0, I1>) -> Self {
        let (val_typ, i0_typ, i1_typ) = (V::typ(), I0::typ(), I1::typ());
        let data = unsafe {
            match val_typ {
                ValTyp::Int => UFunP2 {
                    int: UFunP2_V::create::<I0, I1>(transmute(fun)),
                },
                ValTyp::Num => UFunP2 {
                    num: UFunP2_V::create::<I0, I1>(transmute(fun)),
                },
            }
        };
        Self::new(val_typ, i0_typ, i1_typ, data)
    }
}
