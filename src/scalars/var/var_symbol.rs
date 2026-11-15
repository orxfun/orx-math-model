use crate::pars::p1::{DP1, FP1};
use crate::scalars::{DSca, DVar};
use crate::sets::IndexCatInt;
use crate::{Ptr, Res, Set, SetCatInt, Symbol, Val, ScaInt};
use alloc::boxed::Box;
use core::ops::Deref;
use core::ops::Index;

#[derive(derive_more::From, Clone, Copy)]
pub struct Var<'m, 'd>(pub(crate) Symbol<'m, 'd, DSca<'d>>);

impl<'m, 'd> Var<'m, 'd> {
    pub(crate) fn data(&self) -> &DVar<'d> {
        unsafe { &self.0.el.get().data.var }
    }
}
