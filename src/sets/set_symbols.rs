// use crate::idxs::{CatIdxInt, CatIdxStr, OrdIdx};
use crate::sets::DSet;
use crate::{Elements, MetaCatInt, MetaCatStr, MetaOrd, MetaSet, Res, Symbol};
use alloc::{boxed::Box, string::String};

fn bind<'d, S, I>(set: Symbol<'_, 'd, DSet<'d>>, fun: impl Fn() -> I + 'd) -> Res
where
    S: MetaSet,
    I: IntoIterator<Item = S::IdxRaw<'d>>,
{
    let fun = Box::new(move |e: &mut Elements<S::IdxRaw<'d>>| e.fill(fun()));
    let data = unsafe { set.el.get() };
    data.bind::<S>(fun)
}

// cat int

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct CatSetInt<'m, 'd>(pub(crate) Symbol<'m, 'd, DSet<'d>>);

impl<'m, 'd> CatSetInt<'m, 'd> {
    pub fn bind<I: IntoIterator<Item = usize>>(self, fun: impl Fn() -> I + 'd) -> Res {
        bind::<MetaCatInt, _>(self.0, fun)
    }

    // pub fn idx(self) -> CatIdxInt<'m, 'd> {
    //     self.m.cat_idx_int(self)
    // }
}

// cat str

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct CatSetStr<'m, 'd>(pub(crate) Symbol<'m, 'd, DSet<'d>>);

impl<'m, 'd> CatSetStr<'m, 'd> {
    pub fn bind<I: IntoIterator<Item = &'d String>>(self, fun: impl Fn() -> I + 'd) -> Res {
        bind::<MetaCatStr, _>(self.0, fun)
    }

    // pub fn idx(self) -> CatIdxStr<'m, 'd> {
    //     self.m.cat_idx_str(self)
    // }
}

// ord

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct OrdSet<'m, 'd>(pub(crate) Symbol<'m, 'd, DSet<'d>>);

impl<'m, 'd> OrdSet<'m, 'd> {
    pub fn bind<I: IntoIterator<Item = i64>>(self, fun: impl Fn() -> I + 'd) -> Res {
        bind::<MetaOrd, _>(self.0, fun)
    }

    // pub fn idx(self) -> OrdIdx<'m, 'd> {
    //     self.m.ord_idx(self)
    // }
}
