use crate::constraints::{Constraint, DCon, IneqTyp};
use crate::idxs::ConIdxs;
use crate::{LinExpr, SymWrap};

pub struct Forall<'m, 'd>(pub(crate) ConIdxs<'m, 'd>);

pub fn forall<'m, 'd>(indices: impl Into<ConIdxs<'m, 'd>>) -> Forall<'m, 'd> {
    Forall(indices.into())
}

pub fn con<'m, 'd>(
    lhs: LinExpr<'m, 'd>,
    typ: IneqTyp,
    rhs: LinExpr<'m, 'd>,
    forall: Forall<'m, 'd>,
) -> Constraint<'m, 'd> {
    let data = DCon::new(forall.0.to_idx_vec(), lhs.ptr(), rhs.ptr(), typ);
    todo!()
}
