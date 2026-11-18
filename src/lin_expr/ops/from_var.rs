use crate::lin_expr::{DLinExpr, DLinTerm, ExprVec};
use crate::{LinExpr, SymWrap, Var};
use core::ops::Neg;

// var

impl<'m, 'd> From<Var<'m, 'd>> for DLinTerm<'d> {
    fn from(value: Var<'m, 'd>) -> Self {
        DLinTerm::new(None, value.ptr())
    }
}

impl<'m, 'd> From<Var<'m, 'd>> for LinExpr<'m, 'd> {
    fn from(value: Var<'m, 'd>) -> Self {
        let terms = ExprVec::new([value.into()]);
        let data = DLinExpr::new(terms, None);
        value.m().lin_exp(data)
    }
}

// neg-var

impl<'m, 'd> Neg for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn neg(self) -> Self::Output {
        let term = DLinTerm::new(Some(self.m().minus_one().ptr()), self.ptr());
        let terms = ExprVec::new([term]);
        let data = DLinExpr::new(terms, None);
        self.m().lin_exp(data)
    }
}
