use crate::lin_expr::le_data::ExprVec;
use crate::lin_expr::{DLinExpr, DLinTerm};
use crate::{LinExpr, Var};

// var

impl<'m, 'd> From<Var<'m, 'd>> for LinExpr<'m, 'd> {
    fn from(value: Var<'m, 'd>) -> Self {
        let terms = ExprVec::new([DLinTerm::new(None, value.ptr())]);
        let data = DLinExpr::new(terms, None);
        value.m().lin_exp(data)
    }
}
