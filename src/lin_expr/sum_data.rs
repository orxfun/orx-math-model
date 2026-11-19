use crate::lin_expr::DLinExpr;
use crate::{Model, Ptr, SetVec};

#[derive(derive_new::new, Clone)]
pub struct DSum<'d> {
    pub sets: SetVec<'d>,
    pub expr: Ptr<'d, DLinExpr<'d>>,
}

impl<'d> DSum<'d> {
    pub fn neg<'m>(&self, m: &'m Model<'d>) -> Self {
        Self::new(self.sets.clone(), self.expr.get().neg(m))
    }

    pub fn with_expr(&self, expr: Ptr<'d, DLinExpr<'d>>) -> Self {
        Self::new(self.sets.clone(), expr)
    }
}
