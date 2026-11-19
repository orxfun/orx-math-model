use crate::constraints::{Constraint, DCon, IneqTyp};
use crate::free_fun::ForAll;
use crate::{LinExpr, SymWrap};

pub fn con<'m, 'd>(
    lhs: LinExpr<'m, 'd>,
    typ: IneqTyp,
    rhs: LinExpr<'m, 'd>,
    forall: ForAll<'m, 'd>,
) -> Constraint<'m, 'd> {
    let data = DCon::new(forall.0.to_idx_vec(), lhs.ptr(), rhs.ptr(), typ);
    todo!()
}
