use crate::pars::p2::{DP2, FP2};
use crate::sets::IndexCatInt;
use crate::{Ptr, Res, Set, SetCatInt, Symbol, Val, ValInt};
use alloc::boxed::Box;
use core::ops::Index;
use core::{marker::PhantomData, ops::Deref};

#[derive(derive_more::From, Clone, Copy)]
pub struct ParD2<'m, 'd, V: Val, I0: Set, I1: Set>(
    pub(crate) Symbol<'m, 'd, DP2<'d>>,
    PhantomData<(V, I0, I1)>,
);

impl<'m, 'd, V: Val, I0: Set, I1: Set> From<Symbol<'m, 'd, DP2<'d>>> for ParD2<'m, 'd, V, I0, I1> {
    fn from(value: Symbol<'m, 'd, DP2<'d>>) -> Self {
        Self(value, PhantomData)
    }
}

impl<'m, 'd, V: Val, I0: Set, I1: Set> Deref for ParD2<'m, 'd, V, I0, I1> {
    type Target = Symbol<'m, 'd, DP2<'d>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'m, 'd, V, I0, I1> ParD2<'m, 'd, V, I0, I1>
where
    V: Val,
    I0: Set,
    I1: Set,
{
    pub fn bind(self, fun: impl Fn(I0::IdxRaw<'d>, I1::IdxRaw<'d>) -> V::Raw + 'd) -> Res {
        let fun: FP2<'d, V, I0, I1> = Box::new(fun);
        let data = unsafe { self.el.get() };
        data.bind::<V, I0, I1>(fun)
    }

    // pub fn at(self, idx: impl Into<I0::IdxAny<'m, 'd>>) -> V::Sym<'m, 'd>
    // where
    //     I0: D1Args,
    // {
    //     <I0 as D1Args>::par_at(self, idx)
    // }
}
