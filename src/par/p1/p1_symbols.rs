use crate::idxs::{Idx, IdxSlot};
use crate::par::p1::p1_bag::DP1;
use crate::sets::SetSlot;
use crate::vals::Val;
use crate::{Elements, ElementsIter, Symbol};
use alloc::{boxed::Box, string::String};
use core::marker::PhantomData;
use core::ops::Deref;

#[derive(Clone, Copy)]
pub struct ParD1<'m, 'd, I0: Idx<'d>, T: Val>(
    pub(crate) Symbol<'m, 'd, DP1<'d, T>>,
    PhantomData<I0>,
);

impl<'m, 'd, I0: Idx<'d>, T: Val> From<Symbol<'m, 'd, DP1<'d, T>>> for ParD1<'m, 'd, I0, T> {
    fn from(value: Symbol<'m, 'd, DP1<'d, T>>) -> Self {
        Self(value, PhantomData)
    }
}

impl<'m, 'd, I0: Idx<'d>, T: Val> Deref for ParD1<'m, 'd, I0, T> {
    type Target = Symbol<'m, 'd, DP1<'d, T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type IntParD1<'m, 'd, I0> = ParD1<'m, 'd, I0, i64>;

pub type NumParD1<'m, 'd, I0> = ParD1<'m, 'd, I0, f64>;
