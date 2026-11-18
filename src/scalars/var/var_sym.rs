use crate::scalars::DVar;
use crate::{impl_sym_wrap, Symbol};

#[derive(derive_new::new, Clone, Copy)]
pub struct Var<'m, 'd>(Symbol<'m, 'd, DVar<'d>>);
impl_sym_wrap!(Var<'m, 'd>, DVar<'d>);
