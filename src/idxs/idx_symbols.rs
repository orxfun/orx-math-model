// use crate::idxs::{CatIdxInt, CatIdxStr, OrdIdx};
use crate::sets::DSet;
use crate::{Elements, MetaCatInt, MetaCatStr, MetaOrd, MetaSet, Res, Symbol};
use alloc::{boxed::Box, string::String};

// cat int

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct IdxCatInt<'m, 'd>(pub(crate) Symbol<'m, 'd, DSet<'d>>);

impl<'m, 'd> IdxCatInt<'m, 'd> {}

// cat str

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct IdxCatStr<'m, 'd>(pub(crate) Symbol<'m, 'd, DSet<'d>>);

impl<'m, 'd> IdxCatStr<'m, 'd> {}

// ord

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct IdxOrd<'m, 'd>(pub(crate) Symbol<'m, 'd, DSet<'d>>);

impl<'m, 'd> IdxOrd<'m, 'd> {}
