use crate::constraints::DCon;
use crate::{impl_sym_wrap, Symbol};

#[derive(derive_new::new, Clone, Copy)]
pub struct Constraint<'m, 'd>(Symbol<'m, 'd, DCon<'d>>);
impl_sym_wrap!(Constraint<'m, 'd>, DCon<'d>);
