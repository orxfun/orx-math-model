use crate::{lin_expr::DLinExpr, Symbol};

#[derive(derive_new::new, Clone, Copy)]
pub struct LinExpr<'m, 'd>(Symbol<'m, 'd, DLinExpr<'d>>);
