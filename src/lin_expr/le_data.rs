use crate::{lin_expr::LtData, ConVec};

pub const MAX_EXPR_LEN: usize = 8;

pub type ExprVec<'d> = ConVec<MAX_EXPR_LEN, LtData<'d>>;

pub struct LeData<'d> {
    terms: ExprVec<'d>,
}
