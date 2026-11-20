use crate::constraints::{Constraint, DCon};
use crate::free_fun::{IneqOp, IntoLinExpr};
use crate::idxs::IdxVec;
use crate::{Model, Set, SetSym, SymWrap};

pub type Idx<'m, 'd, S> = <<S as SetSym<'m, 'd>>::Meta as Set>::IdxSym<'m, 'd>;

pub fn common_m<'m, 'd>(
    lhs_m: Option<&'m Model<'d>>,
    rhs_m: Option<&'m Model<'d>>,
) -> Option<&'m Model<'d>> {
    match (lhs_m, rhs_m) {
        (Some(m1), Some(_m2)) => Some(m1),
        (Some(m1), None) => Some(m1),
        (None, Some(m2)) => Some(m2),
        (None, None) => None,
    }
}

pub fn con<'m, 'd>(
    lhs: impl Into<IntoLinExpr<'m, 'd>>,
    op: impl IneqOp,
    rhs: impl Into<IntoLinExpr<'m, 'd>>,
) -> Constraint<'m, 'd> {
    let idxs = IdxVec::new([]);
    let [lhs, rhs] = [lhs.into(), rhs.into()];
    let m = common_m(lhs.m(), rhs.m()).unwrap(); // TODO: Handle const-const case somehow!
    let [lhs, rhs] = [lhs, rhs].map(|e| e.to_expr(m));
    m.constraint(DCon::new(idxs, lhs.ptr(), rhs.ptr(), op.typ()))
}
