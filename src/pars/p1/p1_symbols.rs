use crate::pars::p1::DP1;
use crate::sets::IndexCatInt;
use crate::val_idx_pairs::{D1Args, D1ValIdxPair};
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
    pub fn bind(self, fun: impl Fn(I0::IdxRaw<'d>) -> V::Raw + 'd) -> Res
    where
        (V, I0): D1ValIdxPair<V = V, I0 = I0>,
    {
        let fun: Box<dyn Fn(I0::IdxRaw<'d>) -> V::Raw + 'd> = Box::new(fun);
        let data = unsafe { self.el.get() };
        data.bind::<V, I0>(fun)
    }

    pub fn at(self, idx: impl Into<I0::IdxAny<'m, 'd>>) -> V::Sym<'m, 'd>
    where
        (V, I0): D1ValIdxPair<V = V, I0 = I0>,
        I0: D1Args,
    {
        <I0 as D1Args>::par_at(self, idx)
    }
}

impl<'m, 'd, V, I0, Idx> Index<Idx> for ParD1<'m, 'd, V, I0>
where
    V: Val,
    I0: Set + D1Args,
    (V, I0): D1ValIdxPair<V = V, I0 = I0>,
    Idx: Into<I0::IdxAny<'m, 'd>>,
{
    type Output = V::Sym<'m, 'd>;

    fn index(&self, idx: Idx) -> &Self::Output {
        let val = self.at(idx);
        self.m.sym_ref::<V>(val)
    }
}
