use crate::scalars::num::DNum;
use crate::{impl_sym_wrap, Symbol};

// int

#[derive(derive_new::new, Clone, Copy)]
pub struct Int<'m, 'd>(Symbol<'m, 'd, DNum<'d>>);
impl_sym_wrap!(Int<'m, 'd>, DNum<'d>);

// num

#[derive(derive_new::new, Clone, Copy)]
pub struct Num<'m, 'd>(Symbol<'m, 'd, DNum<'d>>);
impl_sym_wrap!(Num<'m, 'd>, DNum<'d>);
