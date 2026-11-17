use crate::pars::DPars;
use crate::{impl_sym_wrap, Symbol};

#[derive(derive_new::new, Clone, Copy)]
pub struct IntParD1<'m, 'd>(Symbol<'m, 'd, DPars<'d>>);
impl_sym_wrap!(IntParD1<'m, 'd>, DPars<'d>);

#[derive(derive_new::new, Clone, Copy)]
pub struct NumParD1<'m, 'd>(Symbol<'m, 'd, DPars<'d>>);
impl_sym_wrap!(NumParD1<'m, 'd>, DPars<'d>);
