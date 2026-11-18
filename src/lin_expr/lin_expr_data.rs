use crate::{lin_expr::DLinTerm, scalars::DNum};
use crate::{ConVec, Ptr};

pub const MAX_EXPR_LEN: usize = 8;

pub type ExprVec<'d> = ConVec<MAX_EXPR_LEN, DLinTerm<'d>>;

#[derive(derive_new::new)]
pub struct DLinExpr<'d> {
    pub(super) terms: ExprVec<'d>,
    pub(super) constant: Option<Ptr<'d, DNum<'d>>>,
}
