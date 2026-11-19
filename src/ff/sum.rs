use crate::{ff::ForAll, LinExpr};

pub fn sum<'m, 'd>(forall: ForAll<'m, 'd>, expr: LinExpr<'m, 'd>) -> LinExpr<'m, 'd> {
    let (m, idxs) = forall.0.destruct();
    // let m = idxs
    todo!()
}
