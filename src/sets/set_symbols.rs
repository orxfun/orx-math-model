use crate::idxs::{IdxCatInt, IdxCatStr, IdxOrd};
use crate::sets::DSet;
use crate::{Elements, Model, Ptr, Res, Set, SetCatInt, SetCatStr, SetOrd, SymWrap, Symbol};
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

impl<'m, 'd> SymWrap<'m, 'd> for CatInt<'m, 'd> {
    type Data = DSet<'d>;

    fn sym(self) -> Symbol<'m, 'd, Self::Data> {
        self.0
    }
}

impl<'m, 'd> CatInt<'m, 'd> {
    pub(crate) fn m(&self) -> &'m Model<'d> {
        self.0.m
    }

    pub(crate) fn data(&self) -> &DSet<'d> {
        unsafe { self.0.el.get() }
    }

    pub(crate) fn ptr(&self) -> Ptr<'d, DSet<'d>> {
        Ptr::new_from_el(self.0.el)
    }

    pub fn bind<I: IntoIterator<Item = usize>>(self, fun: impl Fn() -> I + 'd) -> Res {
        bind::<SetCatInt, _>(self.0, fun)
    }

    pub fn idx(self) -> IdxCatInt<'m, 'd> {
        self.0.m.idx::<SetCatInt>(self)
    }
}

// cat str

#[derive(derive_new::new, Clone, Copy)]
pub struct CatStr<'m, 'd>(Symbol<'m, 'd, DSet<'d>>);

impl<'m, 'd> SymWrap<'m, 'd> for CatStr<'m, 'd> {
    type Data = DSet<'d>;

    fn sym(self) -> Symbol<'m, 'd, Self::Data> {
        self.0
    }
}

impl<'m, 'd> CatStr<'m, 'd> {
    pub(crate) fn m(&self) -> &'m Model<'d> {
        self.0.m
    }

    pub(crate) fn data(&self) -> &DSet<'d> {
        unsafe { self.0.el.get() }
    }

    pub(crate) fn ptr(&self) -> Ptr<'d, DSet<'d>> {
        Ptr::new_from_el(self.0.el)
    }

    pub fn bind<I: IntoIterator<Item = &'d String>>(self, fun: impl Fn() -> I + 'd) -> Res {
        bind::<SetCatStr, _>(self.0, fun)
    }

    pub fn idx(self) -> IdxCatStr<'m, 'd> {
        self.0.m.idx::<SetCatStr>(self)
    }
}

// ord

#[derive(derive_new::new, Clone, Copy)]
pub struct Ord<'m, 'd>(Symbol<'m, 'd, DSet<'d>>);

impl<'m, 'd> SymWrap<'m, 'd> for Ord<'m, 'd> {
    type Data = DSet<'d>;

    fn sym(self) -> Symbol<'m, 'd, Self::Data> {
        self.0
    }
}

impl<'m, 'd> Ord<'m, 'd> {
    pub(crate) fn m(&self) -> &'m Model<'d> {
        self.0.m
    }

    pub(crate) fn data(&self) -> &DSet<'d> {
        unsafe { self.0.el.get() }
    }

    pub(crate) fn ptr(&self) -> Ptr<'d, DSet<'d>> {
        Ptr::new_from_el(self.0.el)
    }

    pub fn bind<I: IntoIterator<Item = i64>>(self, fun: impl Fn() -> I + 'd) -> Res {
        bind::<SetOrd, _>(self.0, fun)
    }

    pub fn idx(self) -> IdxOrd<'m, 'd> {
        self.0.m.idx::<SetOrd>(self)
    }
}
