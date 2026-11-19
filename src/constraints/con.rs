use crate::LinExpr;

macro_rules! con {
    ($lhs:expr, $typ:ident, $rhs:expr) => {};
}

fn abc<'m, 'd>(l: LinExpr<'m, 'd>, r: LinExpr<'m, 'd>) {
    con!(l, l, r);
}
