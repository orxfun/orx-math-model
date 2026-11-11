use crate::sets::{SetFun, SetSlot};
use crate::{Elements, Res, Symbol};
use alloc::{boxed::Box, string::String};

// cat int

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct CatSetInt<'m, 'd>(pub(crate) Symbol<'m, 'd, SetSlot<'d>>);

impl<'m, 'd> CatSetInt<'m, 'd> {
    pub fn bind<I: IntoIterator<Item = usize>>(self, fun: impl Fn() -> I + 'd) -> Res {
        let fun = Box::new(move |e: &mut Elements<usize>| e.fill(fun()));
        let slot = unsafe { self.el.get() };
        slot.bind(SetFun::CatSetInt(fun))
    }
}

// cat str

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct CatSetStr<'m, 'd>(pub(crate) Symbol<'m, 'd, SetSlot<'d>>);

impl<'m, 'd> CatSetStr<'m, 'd> {
    pub fn bind<I: IntoIterator<Item = &'d String>>(self, fun: impl Fn() -> I + 'd) -> Res {
        let fun = Box::new(move |e: &mut Elements<&'d String>| e.fill(fun()));
        let slot = unsafe { self.el.get() };
        slot.bind(SetFun::CatSetStr(fun))
    }
}

// ord

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct OrdSet<'m, 'd>(pub(crate) Symbol<'m, 'd, SetSlot<'d>>);

impl<'m, 'd> OrdSet<'m, 'd> {
    pub fn bind<I: IntoIterator<Item = i64>>(self, fun: impl Fn() -> I + 'd) -> Res {
        let fun = Box::new(move |e: &mut Elements<i64>| e.fill(fun()));
        let slot = unsafe { self.el.get() };
        slot.bind(SetFun::OrdSet(fun))
    }
}
