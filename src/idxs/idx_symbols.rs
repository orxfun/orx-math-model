// use crate::idxs::{CatIdxInt, CatIdxStr, OrdIdx};
use crate::idxs::DIdx;
use crate::{Elements, MetaCatInt, MetaCatStr, MetaOrd, Set, Res, Symbol};
use alloc::{boxed::Box, string::String};

// cat int

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct IdxCatInt<'m, 'd>(pub(crate) Symbol<'m, 'd, DIdx<'d>>);

impl<'m, 'd> IdxCatInt<'m, 'd> {}

// cat str

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct IdxCatStr<'m, 'd>(pub(crate) Symbol<'m, 'd, DIdx<'d>>);

impl<'m, 'd> IdxCatStr<'m, 'd> {}

// ord

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct IdxOrd<'m, 'd>(pub(crate) Symbol<'m, 'd, DIdx<'d>>);

impl<'m, 'd> IdxOrd<'m, 'd> {}
