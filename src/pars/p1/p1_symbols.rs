use crate::pars::p1::DP1;
use crate::{Res, Set, Symbol, Val};
use alloc::boxed::Box;
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

impl<'m, 'd, V: Val, I0: Set> ParD1<'m, 'd, V, I0> {
    pub fn bind(self, fun: impl Fn(I0::IdxRaw<'d>) -> V::Raw + 'd) -> Res {
        let fun: Box<dyn Fn(I0::IdxRaw<'d>) -> V::Raw + 'd> = Box::new(fun);
        let data = unsafe { self.el.get() };
        data.bind::<V, I0>(fun)
    }
}
