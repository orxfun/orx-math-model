use crate::{Elements, Res, ResValid, Set, SetCatInt, SetCatStr, SetOrd, SetTyp};
use alloc::boxed::Box;
use core::mem::{transmute, ManuallyDrop};

pub type FSet<'d, S> = Box<dyn Fn(&mut Elements<<S as Set>::IdxRaw<'d>>) + 'd>;

#[repr(C)]
pub union UFunSet<'d> {
    pub cint: ManuallyDrop<FSet<'d, SetCatInt>>,
    pub cstr: ManuallyDrop<FSet<'d, SetCatStr>>,
    pub ord: ManuallyDrop<FSet<'d, SetOrd>>,
}

pub struct FunSet<'d> {
    pub typ: SetTyp,
    data: UFunSet<'d>,
}

impl<'d> Drop for FunSet<'d> {
    fn drop(&mut self) {
        unsafe {
            match self.typ {
                SetTyp::CatInt => ManuallyDrop::drop(&mut self.data.cint),
                SetTyp::CatStr => ManuallyDrop::drop(&mut self.data.cstr),
                SetTyp::Ord => ManuallyDrop::drop(&mut self.data.ord),
            }
        }
    }
}

impl<'d> FunSet<'d> {
    pub fn empty(typ: SetTyp) -> Self {
        let data = match typ {
            SetTyp::CatInt => UFunSet {
                cint: ManuallyDrop::new(Box::new(|_| {})),
            },
            SetTyp::CatStr => UFunSet {
                cstr: ManuallyDrop::new(Box::new(|_| {})),
            },
            SetTyp::Ord => UFunSet {
                ord: ManuallyDrop::new(Box::new(|_| {})),
            },
        };
        Self { typ, data }
    }

    pub fn new<S: Set>(fun: FSet<'d, S>) -> Self {
        let fun = ManuallyDrop::new(fun);
        let typ = S::typ();
        let data = match typ {
            SetTyp::CatInt => UFunSet {
                cint: unsafe { transmute(fun) },
            },
            SetTyp::CatStr => UFunSet {
                cstr: unsafe { transmute(fun) },
            },
            SetTyp::Ord => UFunSet {
                ord: unsafe { transmute(fun) },
            },
        };
        Self { typ, data }
    }

    pub fn get<S: Set>(&self) -> &FSet<'d, S> {
        Res::dbg_assert_same_set(S::typ(), self.typ);
        S::set_unpack(&self.data)
    }
}
