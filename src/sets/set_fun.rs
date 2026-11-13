use crate::{CatInt, CatStr, Elements, Ord, Res, ResValid, Set, SetTyp};
use alloc::boxed::Box;
use core::mem::{transmute, ManuallyDrop};

pub type FSet<'d, S> = Box<dyn Fn(&mut Elements<<S as Set>::IdxRaw<'d>>) + 'd>;

#[repr(C)]
pub union UFunSet<'d> {
    pub cat_int: ManuallyDrop<FSet<'d, CatInt>>,
    pub cat_str: ManuallyDrop<FSet<'d, CatStr>>,
    pub ord: ManuallyDrop<FSet<'d, Ord>>,
}

pub struct FunSet<'d> {
    pub typ: SetTyp,
    data: UFunSet<'d>,
}

impl<'d> Drop for FunSet<'d> {
    fn drop(&mut self) {
        unsafe {
            match self.typ {
                SetTyp::CatInt => ManuallyDrop::drop(&mut self.data.cat_int),
                SetTyp::CatStr => ManuallyDrop::drop(&mut self.data.cat_str),
                SetTyp::Ord => ManuallyDrop::drop(&mut self.data.ord),
            }
        }
    }
}

impl<'d> FunSet<'d> {
    pub fn empty(typ: SetTyp) -> Self {
        let data = match typ {
            SetTyp::CatInt => UFunSet {
                cat_int: ManuallyDrop::new(Box::new(|_| {})),
            },
            SetTyp::CatStr => UFunSet {
                cat_str: ManuallyDrop::new(Box::new(|_| {})),
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
                cat_int: unsafe { transmute(fun) },
            },
            SetTyp::CatStr => UFunSet {
                cat_str: unsafe { transmute(fun) },
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
