use crate::lin_expr::DLinTerm;
use crate::{LinExpr, Var};
use core::ops::Add;

// expr + var

impl<'m, 'd> Add<Var<'m, 'd>> for LinExpr<'m, 'd> {
    type Output = LinExpr<'m, 'd>;

    fn add(self, rhs: Var<'m, 'd>) -> Self::Output {
        let t = DLinTerm::from(rhs);
        todo!()
    }
}
