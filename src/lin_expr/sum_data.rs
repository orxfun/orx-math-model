use crate::lin_expr::DLinExpr;
use crate::{Ptr, SetVec};

#[derive(derive_new::new, Clone)]
pub struct DSum<'d> {
    pub sets: SetVec<'d>,
    pub expr: Ptr<'d, DLinExpr<'d>>,
}
