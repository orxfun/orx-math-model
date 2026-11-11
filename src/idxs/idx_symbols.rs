use crate::idxs::idx_bag::DIdx;
use crate::Symbol;
use alloc::{boxed::Box, string::String};

// cat int

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct CatIdxInt<'m, 'd>(pub(crate) Symbol<'m, 'd, DIdx<'d>>);

// cat str

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct CatIdxStr<'m, 'd>(pub(crate) Symbol<'m, 'd, DIdx<'d>>);

// ord

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct OrdIdx<'m, 'd>(pub(crate) Symbol<'m, 'd, DIdx<'d>>);
