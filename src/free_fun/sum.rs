use crate::lin_expr::{DLinExpr, SumVec, TermVec};
use crate::{free_fun::ForAll, lin_expr::DSum};
use crate::{LinExpr, SymWrap};

pub fn sum<'m, 'd>(forall: ForAll<'m, 'd>, sum_expr: LinExpr<'m, 'd>) -> LinExpr<'m, 'd> {
    let m = sum_expr.m();
    let idxs = forall.0.to_idx_vec();
    let sum = DSum::new(idxs, sum_expr.ptr());
    let sums = SumVec::new([sum]);
    let terms = TermVec::new([]);
    let expr = DLinExpr::new(terms, sums, None);
    m.lin_exp(expr)
}
