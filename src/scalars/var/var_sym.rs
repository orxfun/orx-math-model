use crate::vars::DV;
use crate::{impl_sym_wrap, Symbol};

#[derive(derive_new::new, Clone, Copy)]
pub struct Var<'m, 'd>(Symbol<'m, 'd, DV<'d>>);
impl_sym_wrap!(Var<'m, 'd>, DV<'d>);
