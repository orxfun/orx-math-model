use crate::idxs::IdxSlot;
use crate::par::p1::p1_bag::DP1;
use crate::par::p1::{P1Fun, P1Slot};
use crate::sets::SetSlot;
use crate::vals::{Int, ValFun};
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
    pub fn at(self, i0: I0::IdxSym<'m, 'd>) -> Int<'m, 'd> {
        // TODO: check if i0 and set of the par agrees
        let par_slot = &unsafe { self.el.get() }.par;
        match (par_slot, i0.slot()) {
            (P1Slot::CatSetInt(par), IdxSlot::CatIdxInt(i0)) => {
                let (par, i0) = (par.ptr(), i0.ptr());
                self.m.val_int(move || par.get()(i0.val()))
            }
            (P1Slot::CatSetStr(par), IdxSlot::CatIdxStr(i0)) => {
                let (par, i0) = (par.ptr(), i0.ptr());
                self.m.val_int(move || par.get()(i0.val()))
            }
            (P1Slot::OrdSet(par), IdxSlot::OrdIdx(i0)) => todo!(),
            _ => todo!(),
        }
    }
}

// num

pub type NumParD1<'m, 'd, I0> = ParD1<'m, 'd, I0, MetaNum>;
