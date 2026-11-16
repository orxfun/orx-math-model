use crate::lin_expr::{DLinExpr, DLinTerm, ExprVec};
use crate::{LinExpr, Var};
use core::ops::Neg;

// var

impl<'m, 'd> From<Var<'m, 'd>> for LinExpr<'m, 'd> {
    fn from(value: Var<'m, 'd>) -> Self {
        let term = DLinTerm::new(None, value.ptr());
        let terms = ExprVec::new([term]);
        let data = DLinExpr::new(terms, None);
        value.m().lin_exp(data)
    }
}

// neg-var

impl<'m, 'd> Neg for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn neg(self) -> Self::Output {
        let term = DLinTerm::new(Some(self.m().minus_one_int().ptr()), self.ptr());
        let terms = ExprVec::new([term]);
        let data = DLinExpr::new(terms, None);
        self.m().lin_exp(data)
    }
}
