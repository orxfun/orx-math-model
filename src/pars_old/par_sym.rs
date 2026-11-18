use crate::pars::p1_fun::FP1;
use crate::pars::p2_fun::FP2;
use crate::pars::{DPars, ParFun};
use crate::vars::DVars;
use crate::{Model, Ptr, Res, ScaVar, Set, Symbol, Val, Var};
use alloc::boxed::Box;
use core::marker::PhantomData;
use core::ops::Index;

// d1

#[derive(derive_new::new, Clone, Copy)]
pub struct ParD1<'m, 'd, V: Val, I0: Set>(Symbol<'m, 'd, DPars<'d>>, PhantomData<(V, I0)>);

impl<'m, 'd, V: Val, I0: Set> ParD1<'m, 'd, V, I0> {
    pub(crate) fn m(&self) -> &'m Model<'d> {
        self.0.m
    }

    pub(crate) fn data(&self) -> &DPars<'d> {
        unsafe { self.0.el.get() }
    }

    pub(crate) fn ptr(&self) -> Ptr<'d, DPars<'d>> {
        Ptr::new_from_el(self.0.el)
    }

    pub fn bind(self, fun: impl Fn(I0::IdxRaw<'d>) -> V::Raw + 'd) -> Res {
        let fun: FP1<'d, V, I0> = Box::new(fun);
        self.data().d1_bind::<V, I0>(fun)
    }

    pub fn at(self, i0: impl Into<I0::IdxAny<'m, 'd>>) -> V::Sym<'m, 'd> {
        I0::p1_at(self, i0)
    }
}

impl<'m, 'd, V, I0, Idx0> Index<Idx0> for ParD1<'m, 'd, V, I0>
where
    V: Val,
    I0: Set,
    Idx0: Into<I0::IdxAny<'m, 'd>>,
{
    type Output = V::Sym<'m, 'd>;

    fn index(&self, i0: Idx0) -> &Self::Output {
        let val = self.at(i0);
        self.0.m.sym_ref::<V>(val)
    }
}

// d2

#[derive(derive_new::new, Clone, Copy)]
pub struct ParD2<'m, 'd, V: Val, I0: Set, I1: Set>(
    Symbol<'m, 'd, DPars<'d>>,
    PhantomData<(V, I0, I1)>,
);

impl<'m, 'd, V: Val, I0: Set, I1: Set> ParD2<'m, 'd, V, I0, I1> {
    pub(crate) fn m(&self) -> &'m Model<'d> {
        self.0.m
    }

    pub(crate) fn data(&self) -> &DPars<'d> {
        unsafe { self.0.el.get() }
    }

    pub(crate) fn ptr(&self) -> Ptr<'d, DPars<'d>> {
        Ptr::new_from_el(self.0.el)
    }

    pub fn bind(self, fun: impl Fn(I0::IdxRaw<'d>, I1::IdxRaw<'d>) -> V::Raw + 'd) -> Res {
        let fun: FP2<'d, V, I0, I1> = Box::new(fun);
        self.data().d2_bind::<V, I0, I1>(fun)
    }

    pub fn at(
        self,
        i0: impl Into<I0::IdxAny<'m, 'd>>,
        i1: impl Into<I1::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd> {
        I0::p2_at::<V, I1>(self, i0, i1)
    }
}

impl<'m, 'd, V, I0, I1, Idx0, Idx1> Index<(Idx0, Idx1)> for ParD2<'m, 'd, V, I0, I1>
where
    V: Val,
    I0: Set,
    I1: Set,
    Idx0: Into<I0::IdxAny<'m, 'd>>,
    Idx1: Into<I1::IdxAny<'m, 'd>>,
{
    type Output = V::Sym<'m, 'd>;

    fn index(&self, (i0, i1): (Idx0, Idx1)) -> &Self::Output {
        let val = self.at(i0, i1);
        self.0.m.sym_ref::<V>(val)
    }
}
