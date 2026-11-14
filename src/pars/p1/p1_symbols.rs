use crate::pars::p1::{DP1, FP1};
use crate::sets::IndexCatInt;
use crate::{Ptr, Res, Set, SetCatInt, Symbol, Val, ValInt};
use alloc::boxed::Box;
use core::ops::Index;
use core::{marker::PhantomData, ops::Deref};

#[derive(derive_more::From, Clone, Copy)]
pub struct ParD1<'m, 'd, V: Val, I0: Set>(pub(crate) Symbol<'m, 'd, DP1<'d>>, PhantomData<(V, I0)>);

impl<'m, 'd, V: Val, I0: Set> From<Symbol<'m, 'd, DP1<'d>>> for ParD1<'m, 'd, V, I0> {
    fn from(value: Symbol<'m, 'd, DP1<'d>>) -> Self {
        Self(value, PhantomData)
    }
}

impl<'m, 'd, V: Val, I0: Set> Deref for ParD1<'m, 'd, V, I0> {
    type Target = Symbol<'m, 'd, DP1<'d>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'m, 'd, V, I0> ParD1<'m, 'd, V, I0>
where
    V: Val,
    I0: Set,
{
    pub fn bind(self, fun: impl Fn(I0::IdxRaw<'d>) -> V::Raw + 'd) -> Res {
        let fun: FP1<'d, V, I0> = Box::new(fun);
        let data = unsafe { self.el.get() };
        data.bind::<V, I0>(fun)
    }

    pub fn at(self, i0: impl Into<I0::IdxAny<'m, 'd>>) -> V::Sym<'m, 'd> {
        I0::p1_at(self, i0)
    }
}

impl<'m, 'd, V, I0, Idx> Index<Idx> for ParD1<'m, 'd, V, I0>
where
    V: Val,
    I0: Set,
    Idx: Into<I0::IdxAny<'m, 'd>>,
{
    type Output = V::Sym<'m, 'd>;

    fn index(&self, i0: Idx) -> &Self::Output {
        let val = self.at(i0);
        self.m.sym_ref::<V>(val)
    }
}
