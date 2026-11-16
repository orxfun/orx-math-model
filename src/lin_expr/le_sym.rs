use crate::{lin_expr::DLinExp, Symbol};

#[derive(derive_new::new, Clone, Copy)]
pub struct LinExpr<'m, 'd>(Symbol<'m, 'd, DLinExp<'d>>);
