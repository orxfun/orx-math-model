use crate::{lin_expr::DLinTerm, ConVec};

pub const MAX_EXPR_LEN: usize = 8;

pub type ExprVec<'d> = ConVec<MAX_EXPR_LEN, DLinTerm<'d>>;

pub struct DLinExp<'d> {
    terms: ExprVec<'d>,
}
