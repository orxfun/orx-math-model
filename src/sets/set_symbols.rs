use crate::idxs::{IdxCatInt, IdxCatStr, IdxOrd};
// use crate::idxs::{CatIdxInt, CatIdxStr, OrdIdx};
use crate::sets::DSet;
use crate::{SetCatInt, SetCatStr, Elements, SetOrd, Res, Set, Symbol};
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
pub struct CatInt<'m, 'd>(pub(crate) Symbol<'m, 'd, DSet<'d>>);

impl<'m, 'd> CatInt<'m, 'd> {
    pub fn bind<I: IntoIterator<Item = usize>>(self, fun: impl Fn() -> I + 'd) -> Res {
        bind::<SetCatInt, _>(self.0, fun)
    }

    pub fn idx(self) -> IdxCatInt<'m, 'd> {
        self.m.idx::<SetCatInt>(self)
    }
}

// cat str

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct CatStr<'m, 'd>(pub(crate) Symbol<'m, 'd, DSet<'d>>);

impl<'m, 'd> CatStr<'m, 'd> {
    pub fn bind<I: IntoIterator<Item = &'d String>>(self, fun: impl Fn() -> I + 'd) -> Res {
        bind::<SetCatStr, _>(self.0, fun)
    }

    pub fn idx(self) -> IdxCatStr<'m, 'd> {
        self.m.idx::<SetCatStr>(self)
    }
}

// ord

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct Ord<'m, 'd>(pub(crate) Symbol<'m, 'd, DSet<'d>>);

impl<'m, 'd> Ord<'m, 'd> {
    pub fn bind<I: IntoIterator<Item = i64>>(self, fun: impl Fn() -> I + 'd) -> Res {
        bind::<SetOrd, _>(self.0, fun)
    }

    pub fn idx(self) -> IdxOrd<'m, 'd> {
        self.m.idx::<SetOrd>(self)
    }
}
