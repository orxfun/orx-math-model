use crate::lin_expr::{Coef, DLinExpr, DLinTerm, SumVec, TermVec};
use crate::{LinExpr, SymWrap, Var};
use core::ops::Neg;

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
