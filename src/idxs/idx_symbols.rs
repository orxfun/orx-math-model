use crate::idxs::DIdx;
use crate::{impl_sym_wrap, Symbol};

// cat int

#[derive(derive_new::new, Clone, Copy)]
pub struct IdxCatInt<'m, 'd>(Symbol<'m, 'd, DIdx<'d>>);
impl_sym_wrap!(IdxCatInt<'m, 'd>, DIdx<'d>);

// cat str

#[derive(derive_new::new, Clone, Copy)]
pub struct IdxCatStr<'m, 'd>(Symbol<'m, 'd, DIdx<'d>>);
impl_sym_wrap!(IdxCatStr<'m, 'd>, DIdx<'d>);

// ord

#[derive(derive_new::new, Clone, Copy)]
pub struct IdxOrd<'m, 'd>(Symbol<'m, 'd, DIdx<'d>>);
impl_sym_wrap!(IdxOrd<'m, 'd>, DIdx<'d>);
