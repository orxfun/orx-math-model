use crate::vars::DV;
use crate::{Model, ScaVar, Set, Symbol, Var};
use core::ops::Index;
use core::{marker::PhantomData, ops::Deref};

// d1

#[derive(derive_new::new, Clone, Copy)]
pub struct VarD1<'m, 'd, I0: Set>(Symbol<'m, 'd, DV<'d>>, PhantomData<I0>);

impl<'m, 'd, I0: Set> VarD1<'m, 'd, I0> {
    pub(crate) fn m(&self) -> &'m Model<'d> {
        self.0.m
    }

    pub(crate) fn data(&self) -> &DV<'d> {
        unsafe { self.0.el.get() }
    }
}

// d2

#[derive(derive_new::new, Clone, Copy)]
pub struct VarD2<'m, 'd, I0: Set, I1: Set>(Symbol<'m, 'd, DV<'d>>, PhantomData<(I0, I1)>);

impl<'m, 'd, I0: Set, I1: Set> VarD2<'m, 'd, I0, I1> {
    pub(crate) fn m(&self) -> &'m Model<'d> {
        self.0.m
    }

    pub(crate) fn data(&self) -> &DV<'d> {
        unsafe { self.0.el.get() }
    }
}
