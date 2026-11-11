use crate::idxs::{CatIdxInt, CatIdxStr, OrdIdx};
use crate::vals::{ValFun, ValSlot};
use crate::{Elements, Res, Symbol};
use alloc::{boxed::Box, string::String};

// int

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct Int<'m, 'd>(pub(crate) Symbol<'m, 'd, ValSlot<'d>>);

impl<'m, 'd> Int<'m, 'd> {
    pub fn bind(self, fun: impl Fn() -> i64 + 'd) -> Res {
        let slot = unsafe { self.el.get() };
        slot.bind(ValFun::Int(Box::new(fun)))
    }
}

// num

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct Num<'m, 'd>(pub(crate) Symbol<'m, 'd, ValSlot<'d>>);

impl<'m, 'd> Num<'m, 'd> {
    pub fn bind(self, fun: impl Fn() -> f64 + 'd) -> Res {
        let slot = unsafe { self.el.get() };
        slot.bind(ValFun::Num(Box::new(fun)))
    }
}
