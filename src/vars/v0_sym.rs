use crate::vars::DVars;
use crate::{impl_sym_wrap, SymWrap, Symbol};

#[derive(derive_new::new, Clone, Copy)]
pub struct VarD0<'m, 'd>(Symbol<'m, 'd, DVars<'d>>);
impl_sym_wrap!(VarD0<'m, 'd>, DVars<'d>);
