use crate::pars::p1::{DP1, FP1};
use crate::sets::IndexCatInt;
use crate::var::DV;
use crate::{Ptr, Res, Set, SetCatInt, Symbol, Val, ValInt};
use alloc::boxed::Box;
use core::ops::Deref;
use core::ops::Index;

#[derive(derive_more::From, Clone, Copy)]
pub struct Var<'m, 'd>(pub(crate) Symbol<'m, 'd, DV<'d>>);

impl<'m, 'd> Deref for Var<'m, 'd> {
    type Target = Symbol<'m, 'd, DV<'d>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
