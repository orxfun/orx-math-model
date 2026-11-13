use crate::idxs::{CatIdxInt, CatIdxStr, OrdIdx};
use crate::vals::ValFun;
use crate::{Elements, Res, Symbol};
use alloc::{boxed::Box, string::String};

// int

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct Int<'m, 'd>(pub(crate) Symbol<'m, 'd, ValFun<'d>>);

impl<'m, 'd> Int<'m, 'd> {}

// num

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct Num<'m, 'd>(pub(crate) Symbol<'m, 'd, ValFun<'d>>);

impl<'m, 'd> Num<'m, 'd> {}
