use crate::constraints::DIneq;
use crate::{impl_sym_wrap, Symbol};

#[derive(derive_new::new, Clone, Copy)]
pub struct Ineq<'m, 'd>(Symbol<'m, 'd, DIneq<'d>>);
impl_sym_wrap!(Ineq<'m, 'd>, DIneq<'d>);
