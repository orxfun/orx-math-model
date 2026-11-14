use crate::pars::p2::DP2;
use crate::sets::IndexCatInt;
use crate::val_idx_pairs::D1Args;
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
