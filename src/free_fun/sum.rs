use crate::idxs::{IdxVec, SumIdxs};
use crate::lin_expr::DSum;
use crate::lin_expr::{DLinExpr, SumVec, TermVec};
use crate::{LinExpr, SymWrap};

pub struct SumOver<'m, 'd>(pub(crate) SumIdxs<'m, 'd>);

pub fn over<'m, 'd>(indices: impl Into<SumIdxs<'m, 'd>>) -> SumOver<'m, 'd> {
    SumOver(indices.into())
}

pub fn sum<'m, 'd>(over: SumOver<'m, 'd>, sum_expr: impl Into<LinExpr<'m, 'd>>) -> LinExpr<'m, 'd> {
    let sum_expr = sum_expr.into();
    let m = sum_expr.m();
    let idxs = over.0.to_idx_vec();
    let sum = DSum::new(idxs, sum_expr.ptr());
    let sums = SumVec::new([sum]);
    let terms = TermVec::new([]);
    let expr = DLinExpr::new(terms, sums, None);
    m.lin_exp(expr)
}

fn sum_gen<'m, 'd>(sum_expr: impl Into<LinExpr<'m, 'd>>, idxs: IdxVec<'d>) -> LinExpr<'m, 'd> {
    let sum_expr = sum_expr.into();
    let m = sum_expr.m();
    let sum = DSum::new(idxs, sum_expr.ptr());
    let sums = SumVec::new([sum]);
    let terms = TermVec::new([]);
    let expr = DLinExpr::new(terms, sums, None);
    m.lin_exp(expr)
}

// pub fn sum1<'m, 'd, S0: SetSym<'m, 'd>>(
//     s0: S0,
//     con: impl Fn(Idx<'m, 'd, S0>) -> Constraint<'m, 'd>,
// ) -> Constraint<'m, 'd> {
//     con(s0.idx())
// }
