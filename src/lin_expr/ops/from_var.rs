use crate::lin_expr::{Coef, DLinExpr, DLinTerm, SumVec, TermVec};
use crate::{LinExpr, SymWrap, Var};
use core::ops::Neg;

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
