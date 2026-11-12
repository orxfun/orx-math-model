use crate::arb_val::ArbVal;
use crate::idxs::IdxSlot;
use crate::par::p1::p1_bag::DP1;
use crate::par::p1::{P1Fun, P1Slot};
use crate::sets::SetSlot;
use crate::vals::{Int, ValFun};
use crate::{
    CatIntIdxSum, CatStrIdxSum, Elements, ElementsIter, Idx, MetaInt, MetaNum, MetaSet, MetaVal,
    OrdIdxSum, SetTyp, Symbol,
};
use alloc::string::ToString;
use alloc::{boxed::Box, string::String};
use core::marker::PhantomData;
use core::mem::transmute;
use core::ops::{Deref, Index};
use orx_imp_vec::Elem;

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

    pub fn at2(self, i0: impl Into<I0::IdxSum<'m, 'd>>) -> V::Sym<'m, 'd> {
        // TODO: check if i0 and set of the par agrees
        let par = unsafe { self.el.get() }.par.fun_ptr::<I0>();
        let x = I0::p1_scalar_fun::<'m, 'd, V>(self.m, i0.into(), par);
        // match i0.into() {
        //     _ => todo!(),
        // }

        //
        todo!()
    }

    pub fn at(self, i0: impl Into<I0::IdxSum<'m, 'd>>) -> V::Sym<'m, 'd> {
        // TODO: check if i0 and set of the par agrees
        let par_slot = &unsafe { self.el.get() }.par;

        match par_slot {
            P1Slot::CatSetInt(par) => match I0::idx_sum_into_cat_int(i0.into()) {
                CatIntIdxSum::Usize(i0) => {
                    let par = par.ptr();
                    self.m.val::<V>(move || par.get()(i0))
                }
                CatIntIdxSum::Idx(i0) => match &unsafe { i0.el.get() }.idx {
                    IdxSlot::CatIdxInt(i0) => {
                        let (par, i0) = (par.ptr(), i0.ptr());
                        self.m.val::<V>(move || par.get()(i0.val()))
                    }
                    _ => unreachable!(),
                },
            },

            P1Slot::CatSetStr(par) => match I0::idx_sum_into_cat_str(i0.into()) {
                CatStrIdxSum::RefString(i0) => {
                    let par = par.ptr();
                    self.m.val::<V>(move || par.get()(i0))
                }
                CatStrIdxSum::String(i0) => {
                    let par = par.ptr();
                    let i0 = unsafe { self.m.arb_val(i0).get() }.as_string_ptr();
                    self.m.val::<V>(move || par.get()(unsafe { &*i0 }))
                }
                CatStrIdxSum::Str(i0) => {
                    let par = par.ptr();
                    let i0 = unsafe { self.m.arb_val(i0.to_string()).get() }.as_string_ptr();
                    self.m.val::<V>(move || par.get()(unsafe { &*i0 }))
                }
                CatStrIdxSum::Idx(i0) => match &unsafe { i0.el.get() }.idx {
                    IdxSlot::CatIdxStr(i0) => {
                        let (par, i0) = (par.ptr(), i0.ptr());
                        self.m.val::<V>(move || par.get()(i0.val()))
                    }
                    _ => unreachable!(),
                },
            },

            P1Slot::OrdSet(par) => match I0::idx_sum_into_ord(i0.into()) {
                OrdIdxSum::Raw(i0) => {
                    let par = par.ptr();
                    self.m.val::<V>(move || par.get()(i0))
                }
                OrdIdxSum::Idx(i0) => match &unsafe { i0.el.get() }.idx {
                    IdxSlot::OrdIdx(i0) => {
                        let (par, i0) = (par.ptr(), i0.ptr());
                        self.m.val::<V>(move || par.get()(i0.val()))
                    }
                    _ => unreachable!(),
                },
                OrdIdxSum::Int(i0) => match unsafe { i0.el.get() } {
                    ValFun::Int(i0) => {
                        let par = par.ptr();
                        let i0: *const dyn Fn() -> i64 = i0;
                        self.m.val::<V>(move || {
                            let i0 = unsafe { &*i0 }();
                            par.get()(i0)
                        })
                    }
                    _ => unreachable!(),
                },
            },
        }
    }
}

impl<'m, 'd, I0, V, X> Index<X> for ParD1<'m, 'd, I0, V>
where
    I0: MetaSet,
    V: MetaVal,
    X: Into<I0::IdxSum<'m, 'd>>,
{
    type Output = V::Sym<'m, 'd>;

    fn index(&self, i0: X) -> &Self::Output {
        self.m.val_sym::<V>(self.at(i0))
    }
}

// int

pub type IntParD1<'m, 'd, I0> = ParD1<'m, 'd, I0, MetaInt>;

// num

pub type NumParD1<'m, 'd, I0> = ParD1<'m, 'd, I0, MetaNum>;
