use crate::constraints::{Constraint, DCon};
use crate::free_fun::con0::{common_m, Idx};
use crate::free_fun::{IneqOp, IntoLinExpr};
use crate::idxs::IdxVec;
use crate::{SetSym, SymWrap};

pub fn for1<'m, 'd, S0, Lhs, Op, Rhs, F>(s0: S0, con: F) -> Constraint<'m, 'd>
where
    S0: SetSym<'m, 'd>,
    Lhs: Into<IntoLinExpr<'m, 'd>>,
    Op: IneqOp,
    Rhs: Into<IntoLinExpr<'m, 'd>>,
    F: Fn(Idx<'m, 'd, S0>) -> (Lhs, Op, Rhs),
{
    let i0 = s0.idx();
    let idxs = IdxVec::new([i0.ptr()]);
    let (lhs, op, rhs) = con(i0);

    let [lhs, rhs] = [lhs.into(), rhs.into()];
    let m = common_m(lhs.m(), rhs.m()).unwrap(); // TODO: Handle const-const case somehow!
    let [lhs, rhs] = [lhs, rhs].map(|e| e.to_expr(m));
    m.constraint(DCon::new(idxs, lhs.ptr(), rhs.ptr(), op.typ()))
}
