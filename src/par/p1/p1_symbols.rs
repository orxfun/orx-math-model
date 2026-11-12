use crate::idxs::IdxSlot;
use crate::par::p1::p1_bag::DP1;
use crate::par::p1::{P1Fun, P1Slot};
use crate::sets::SetSlot;
use crate::vals::{Int, ValFun};
use crate::{
    CatIntIndexer, CatStrIndexer, Elements, ElementsIter, Idx, MetaInt, MetaNum, MetaSet, MetaVal,
    OrdIndexer, SetTyp, Symbol,
};
use alloc::{boxed::Box, string::String};
use core::marker::PhantomData;
use core::mem::transmute;
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
    pub fn bind(&self, fun: impl Fn(I0::IdxRaw<'d>) -> V::Val + 'd) {
        let slot = &unsafe { self.el.get() }.par;
        let fun: P1Fun<'_, V> = I0::into_p1_fun(fun);
        slot.bind(fun).expect("TODO: it must succeed, why")
    }
}

// int

pub type IntParD1<'m, 'd, I0> = ParD1<'m, 'd, I0, MetaInt>;

impl<'m, 'd, I0: MetaSet> IntParD1<'m, 'd, I0> {
    pub fn at(self, i0: impl Into<I0::IdxSum<'m, 'd>>) -> Int<'m, 'd> {
        // TODO: check if i0 and set of the par agrees
        let par_slot = &unsafe { self.el.get() }.par;

        match par_slot {
            P1Slot::CatSetInt(par) => match I0::idx_sum_into_cat_int(i0.into()) {
                CatIntIndexer::Raw(i0) => {
                    let par = par.ptr();
                    self.m.val_int(move || par.get()(i0))
                }
                CatIntIndexer::Idx(i0) => match &unsafe { i0.el.get() }.idx {
                    IdxSlot::CatIdxInt(i0) => {
                        let (par, i0) = (par.ptr(), i0.ptr());
                        self.m.val_int(move || par.get()(i0.val()))
                    }
                    _ => unreachable!(),
                },
            },

            P1Slot::CatSetStr(par) => match I0::idx_sum_into_cat_str(i0.into()) {
                CatStrIndexer::Raw(i0) => {
                    let par = par.ptr();
                    self.m.val_int(move || par.get()(i0))
                }
                CatStrIndexer::Idx(i0) => match &unsafe { i0.el.get() }.idx {
                    IdxSlot::CatIdxStr(i0) => {
                        let (par, i0) = (par.ptr(), i0.ptr());
                        self.m.val_int(move || par.get()(i0.val()))
                    }
                    _ => unreachable!(),
                },
            },

            P1Slot::OrdSet(par) => match I0::idx_sum_into_ord(i0.into()) {
                OrdIndexer::Raw(i0) => {
                    let par = par.ptr();
                    self.m.val_int(move || par.get()(i0))
                }
                OrdIndexer::Idx(i0) => match &unsafe { i0.el.get() }.idx {
                    IdxSlot::OrdIdx(i0) => {
                        let (par, i0) = (par.ptr(), i0.ptr());
                        self.m.val_int(move || par.get()(i0.val()))
                    }
                    _ => unreachable!(),
                },
                OrdIndexer::Int(i0) => match unsafe { i0.el.get() } {
                    ValFun::Int(i0) => {
                        let par = par.ptr();
                        let i0: *const dyn Fn() -> i64 = i0;
                        self.m.val_int(move || {
                            let i0 = unsafe { &*i0 }();
                            par.get()(i0)
                        })
                    }
                    _ => unreachable!(),
                },
            },

            _ => todo!(),
        }

        // match (par_slot, i0) {
        //     // (P1Slot::CatSetInt(par), IdxSlot::CatIdxInt(i0)) => {
        //     //     let (par, i0) = (par.ptr(), i0.ptr());
        //     //     self.m.val_int(move || par.get()(i0.val()))
        //     // }
        //     // (P1Slot::CatSetStr(par), IdxSlot::CatIdxStr(i0)) => {
        //     //     let (par, i0) = (par.ptr(), i0.ptr());
        //     //     self.m.val_int(move || par.get()(i0.val()))
        //     // }
        //     // (P1Slot::OrdSet(par), IdxSlot::OrdIdx(i0)) => todo!(),

        //     //
        //     _ => todo!(),
        // }
    }
}

// num

pub type NumParD1<'m, 'd, I0> = ParD1<'m, 'd, I0, MetaNum>;
