use crate::free_fun::IntoLinExpr;
use crate::idxs::IdxVec;
use crate::lin_expr::{DLinExpr, DSum, SumVec, TermVec};
use crate::{LinExpr, Model, Set, SetSym, SymWrap};

fn sum_gen<'m, 'd>(
    m: &'m Model<'d>,
    sum_expr: impl Into<IntoLinExpr<'m, 'd>>,
    idxs: IdxVec<'d>,
) -> LinExpr<'m, 'd> {
    let sum_expr = sum_expr.into();
    let sum_expr = sum_expr.to_expr(m);
    let sum = DSum::new(idxs, sum_expr.ptr());
    let sums = SumVec::new([sum]);
    let terms = TermVec::new([]);
    let expr = DLinExpr::new(terms, sums, None);
    m.lin_exp(expr)
}

type Idx<'m, 'd, S> = <<S as SetSym<'m, 'd>>::Meta as Set>::IdxSym<'m, 'd>;

pub fn sum1<'m, 'd, E: Into<IntoLinExpr<'m, 'd>>, S0: SetSym<'m, 'd>>(
    s0: S0,
    sum_expr: impl Fn(Idx<'m, 'd, S0>) -> E,
) -> LinExpr<'m, 'd> {
    let i0 = s0.idx();
    let idxs = IdxVec::new([i0.ptr()]);
    let sum_expr = sum_expr(i0);
    sum_gen(s0.m(), sum_expr, idxs)
}

pub fn sum2<'m, 'd, E: Into<IntoLinExpr<'m, 'd>>, S0: SetSym<'m, 'd>, S1: SetSym<'m, 'd>>(
    s0: S0,
    s1: S1,
    sum_expr: impl Fn(Idx<'m, 'd, S0>, Idx<'m, 'd, S1>) -> E,
) -> LinExpr<'m, 'd> {
    let i0 = s0.idx();
    let i1 = s1.idx();
    let idxs = IdxVec::new([i0.ptr(), i1.ptr()]);
    let sum_expr = sum_expr(i0, i1);
    sum_gen(s0.m(), sum_expr, idxs)
}
