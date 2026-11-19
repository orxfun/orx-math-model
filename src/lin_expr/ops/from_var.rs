use crate::lin_expr::{Coef, DLinExpr, DLinTerm, TermVec, SumVec};
use crate::{LinExpr, SymWrap, Var};
use core::ops::Neg;

// var

impl<'m, 'd> From<Var<'m, 'd>> for DLinTerm<'d> {
    fn from(value: Var<'m, 'd>) -> Self {
        DLinTerm::new(Coef::One, value.ptr())
    }
}

impl<'m, 'd> From<Var<'m, 'd>> for LinExpr<'m, 'd> {
    fn from(value: Var<'m, 'd>) -> Self {
        let terms = TermVec::new([value.into()]);
        let sums = SumVec::new([]);
        let data = DLinExpr::new(terms, sums, None);
        value.m().lin_exp(data)
    }
}

// neg-var

impl<'m, 'd> Neg for Var<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn neg(self) -> Self::Output {
        let term = DLinTerm::new(Coef::MinusOne, self.ptr());
        let terms = TermVec::new([term]);
        let sums = SumVec::new([]);
        let data = DLinExpr::new(terms, sums, None);
        self.m().lin_exp(data)
    }
}
