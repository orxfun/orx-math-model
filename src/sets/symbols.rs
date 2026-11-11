use crate::{sets::SetSlot, Symbol};

#[derive(derive_more::From, derive_more::Deref)]
pub struct CatSetInt<'m, 'd>(pub(crate) Symbol<'m, 'd, SetSlot<'d>>);

#[derive(derive_more::From, derive_more::Deref)]
pub struct CatSetStr<'m, 'd>(pub(crate) Symbol<'m, 'd, SetSlot<'d>>);

#[derive(derive_more::From, derive_more::Deref)]
pub struct OrdSet<'m, 'd>(pub(crate) Symbol<'m, 'd, SetSlot<'d>>);
