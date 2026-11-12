use crate::idxs::IdxSlot;
use crate::par::p1::p1_bag::DP1;
use crate::par::p1::P1Fun;
use crate::sets::SetSlot;
use crate::vals::Int;
use crate::{Elements, ElementsIter, Idx, MetaInt, MetaNum, MetaSet, MetaVal, SetTyp, Symbol};
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

impl<'m, 'd, I0: MetaSet, V: MetaVal> ParD1<'m, 'd, I0, V> {
    pub fn bind(&self, fun: impl Fn(I0::Idx<'d>) -> V::Val + 'd) {
        let slot = &unsafe { self.el.get() }.par;
        let fun: P1Fun<'_, V> = I0::into_p1_fun(fun);
        slot.bind(fun).expect("TODO: it must succeed, why")
    }
}

// int

pub type IntParD1<'m, 'd, I0> = ParD1<'m, 'd, I0, MetaInt>;

impl<'m, 'd, I0: MetaSet> IntParD1<'m, 'd, I0> {
    pub fn at(self, idx: I0::IdxSym<'m, 'd>) -> Int<'m, 'd> {
        match idx.slot() {
            IdxSlot::CatIdxInt(idx) => {
                // Box<dyn Fn() -> i64 + 'd>
                //
                let fun: Box<dyn Fn() -> i64 + 'd> = Box::new(move || {
                    let x = idx.val();
                    0i64
                });
                todo!()
            }
            IdxSlot::CatIdxStr(idx) => todo!(),
            IdxSlot::OrdIdx(idx) => todo!(),
        }
    }
}

// num

pub type NumParD1<'m, 'd, I0> = ParD1<'m, 'd, I0, MetaNum>;
