use crate::idxs::{IdxCatInt, IdxCatStr, IdxOrd};
// use crate::idxs::{CatIdxInt, CatIdxStr, OrdIdx};
use crate::sets::DSet;
use crate::{Elements, CatInt, CatStr, Ord, Set, Res, Symbol};
use alloc::{boxed::Box, string::String};

fn bind<'d, S, I>(set: Symbol<'_, 'd, DSet<'d>>, fun: impl Fn() -> I + 'd) -> Res
where
    S: Set,
    I: IntoIterator<Item = S::IdxRaw<'d>>,
{
    let fun = Box::new(move |e: &mut Elements<S::IdxRaw<'d>>| e.fill(fun()));
    let data = unsafe { set.el.get() };
    data.bind::<S>(fun)
}

// cat int

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct SetCatInt<'m, 'd>(pub(crate) Symbol<'m, 'd, DSet<'d>>);

impl<'m, 'd> SetCatInt<'m, 'd> {
    pub fn bind<I: IntoIterator<Item = usize>>(self, fun: impl Fn() -> I + 'd) -> Res {
        bind::<CatInt, _>(self.0, fun)
    }

    pub fn idx(self) -> IdxCatInt<'m, 'd> {
        self.m.idx::<CatInt>(self)
    }
}

// cat str

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct SetCatStr<'m, 'd>(pub(crate) Symbol<'m, 'd, DSet<'d>>);

impl<'m, 'd> SetCatStr<'m, 'd> {
    pub fn bind<I: IntoIterator<Item = &'d String>>(self, fun: impl Fn() -> I + 'd) -> Res {
        bind::<CatStr, _>(self.0, fun)
    }

    pub fn idx(self) -> IdxCatStr<'m, 'd> {
        self.m.idx::<CatStr>(self)
    }
}

// ord

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct SetOrd<'m, 'd>(pub(crate) Symbol<'m, 'd, DSet<'d>>);

impl<'m, 'd> SetOrd<'m, 'd> {
    pub fn bind<I: IntoIterator<Item = i64>>(self, fun: impl Fn() -> I + 'd) -> Res {
        bind::<Ord, _>(self.0, fun)
    }

    pub fn idx(self) -> IdxOrd<'m, 'd> {
        self.m.idx::<Ord>(self)
    }
}
