use crate::pars::p1::FP1;
use crate::pars::p2::FP2;
use crate::pars::DPar;
use crate::vars::DV;
use crate::{Model, Res, ScaVar, Set, Symbol, Val, Var};
use alloc::boxed::Box;
use core::marker::PhantomData;
use core::ops::Index;

// d1

#[derive(derive_new::new, Clone, Copy)]
pub struct ParD1<'m, 'd, V: Val, I0: Set>(Symbol<'m, 'd, DPar<'d>>, PhantomData<(V, I0)>);

impl<'m, 'd, V: Val, I0: Set> ParD1<'m, 'd, V, I0> {
    pub(crate) fn data(&self) -> &DPar<'d> {
        unsafe { self.0.el.get() }
    }
    pub fn bind(self, fun: impl Fn(I0::IdxRaw<'d>) -> V::Raw + 'd) -> Res {
        let fun: FP1<'d, V, I0> = Box::new(fun);
        self.data().d1_bind::<V, I0>(fun)
    }
}

// d2

#[derive(derive_new::new, Clone, Copy)]
pub struct ParD2<'m, 'd, V: Val, I0: Set, I1: Set>(
    Symbol<'m, 'd, DPar<'d>>,
    PhantomData<(V, I0, I1)>,
);

impl<'m, 'd, V: Val, I0: Set, I1: Set> ParD2<'m, 'd, V, I0, I1> {
    pub(crate) fn data(&self) -> &DPar<'d> {
        unsafe { self.0.el.get() }
    }
    pub fn bind(self, fun: impl Fn(I0::IdxRaw<'d>, I1::IdxRaw<'d>) -> V::Raw + 'd) -> Res {
        let fun: FP2<'d, V, I0, I1> = Box::new(fun);
        self.data().d2_bind::<V, I0, I1>(fun)
    }
}
