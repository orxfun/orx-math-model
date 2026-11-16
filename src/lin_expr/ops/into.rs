use crate::lin_expr::{DLinExpr, DLinTerm, ExprVec};
use crate::{LinExpr, Var};

// var

impl<'m, 'd> From<Var<'m, 'd>> for LinExpr<'m, 'd> {
    fn from(value: Var<'m, 'd>) -> Self {
        let term = DLinTerm::new(None, value.ptr());
        let terms = ExprVec::new([term]);
        let data = DLinExpr::new(terms, None);
        value.m().lin_exp(data)
    }
}
