use crate::vals::DVal;
use crate::Symbol;

// int

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct Int<'m, 'd>(pub(crate) Symbol<'m, 'd, DVal<'d>>);

// num

#[derive(derive_more::From, derive_more::Deref, Clone, Copy)]
pub struct Num<'m, 'd>(pub(crate) Symbol<'m, 'd, DVal<'d>>);
