use crate::lin_expr::{DLinExpr, DLinTerm, ExprVec};
use crate::{Int, LinExpr, Num, Var};
use core::ops::Mul;

// ? * var

impl<'m, 'd> Mul<Var<'m, 'd>> for Int<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn mul(self, rhs: Var<'m, 'd>) -> Self::Output {
        let term = DLinTerm::new(Some(rhs.ptr()), rhs.ptr());
        let terms = ExprVec::new([term]);
        let data = DLinExpr::new(terms, None);
        self.m().lin_exp(data)
    }
}

impl<'m, 'd> Mul<Var<'m, 'd>> for Num<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn mul(self, rhs: Var<'m, 'd>) -> Self::Output {
        let term = DLinTerm::new(Some(rhs.ptr()), rhs.ptr());
        let terms = ExprVec::new([term]);
        let data = DLinExpr::new(terms, None);
        self.m().lin_exp(data)
    }
}
