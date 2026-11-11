use crate::idxs::IdxSlot;
use crate::par::p1::p1_bag::DP1;
use crate::sets::{MetaSet, SetSlot};
use crate::vals::{MetaInt, MetaVal};
use crate::{Elements, ElementsIter, Symbol};
use alloc::{boxed::Box, string::String};
use core::marker::PhantomData;
use core::ops::Deref;

#[derive(Clone, Copy)]
pub struct ParD1<'m, 'd, I0: MetaSet, V: MetaVal>(
    pub(crate) Symbol<'m, 'd, DP1<'d, V>>,
    PhantomData<I0>,
);

impl<'m, 'd, I0: MetaSet, V: MetaVal> From<Symbol<'m, 'd, DP1<'d, V>>> for ParD1<'m, 'd, I0, V> {
    fn from(value: Symbol<'m, 'd, DP1<'d, V>>) -> Self {
        Self(value, PhantomData)
    }
}

impl<'m, 'd, I0: MetaSet, V: MetaVal> Deref for ParD1<'m, 'd, I0, V> {
    type Target = Symbol<'m, 'd, DP1<'d, V>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// int

pub type IntParD1<'m, 'd, I0> = ParD1<'m, 'd, I0, MetaInt>;

impl<'m, 'd, I0: MetaSet> IntParD1<'m, 'd, I0> {
    pub fn bind(&self, fun: impl Fn(I0) -> i64 + 'd) {
        todo!()
    }
}

// num

pub type NumParD1<'m, 'd, I0> = ParD1<'m, 'd, I0, f64>;
