use crate::{pars::p1::DP1, Set, Symbol, Val};
use core::{marker::PhantomData, ops::Deref};

#[derive(derive_more::From, Clone, Copy)]
pub struct ParD1<'m, 'd, V: Val, I0: Set>(pub(crate) Symbol<'m, 'd, DP1<'d>>, PhantomData<(V, I0)>);

impl<'m, 'd, V: Val, I0: Set> Deref for ParD1<'m, 'd, V, I0> {
    type Target = Symbol<'m, 'd, DP1<'d>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
