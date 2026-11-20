use crate::idxs::IdxOrd;
use crate::sets::DSet;
use crate::{impl_sym_wrap, Elements, Res, Set, SetCatInt, SetCatStr, SetOrd, Symbol};
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

#[derive(derive_new::new, Clone, Copy)]
pub struct CatInt<'m, 'd>(Symbol<'m, 'd, DSet<'d>>);
impl_sym_wrap!(CatInt<'m, 'd>, DSet<'d>);

impl<'m, 'd> CatInt<'m, 'd> {
    pub fn bind<I: IntoIterator<Item = usize>>(self, fun: impl Fn() -> I + 'd) -> Res {
        bind::<SetCatInt, _>(self.0, fun)
    }
}

// cat str

#[derive(derive_new::new, Clone, Copy)]
pub struct CatStr<'m, 'd>(Symbol<'m, 'd, DSet<'d>>);
impl_sym_wrap!(CatStr<'m, 'd>, DSet<'d>);

impl<'m, 'd> CatStr<'m, 'd> {
    pub fn bind<I: IntoIterator<Item = &'d String>>(self, fun: impl Fn() -> I + 'd) -> Res {
        bind::<SetCatStr, _>(self.0, fun)
    }
}

// ord

#[derive(derive_new::new, Clone, Copy)]
pub struct Ord<'m, 'd>(Symbol<'m, 'd, DSet<'d>>);
impl_sym_wrap!(Ord<'m, 'd>, DSet<'d>);

impl<'m, 'd> Ord<'m, 'd> {
    pub fn bind<I: IntoIterator<Item = i64>>(self, fun: impl Fn() -> I + 'd) -> Res {
        bind::<SetOrd, _>(self.0, fun)
    }

    pub fn idx(self) -> IdxOrd<'m, 'd> {
        self.0.m.idx::<SetOrd>(self)
    }
}
