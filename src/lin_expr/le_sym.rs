use crate::lin_expr::DLinExpr;
use crate::{impl_sym_wrap, Symbol};

#[derive(derive_new::new, Clone, Copy)]
pub struct LinExpr<'m, 'd>(Symbol<'m, 'd, DLinExpr<'d>>);
impl_sym_wrap!(LinExpr<'m, 'd>, DLinExpr<'d>);
