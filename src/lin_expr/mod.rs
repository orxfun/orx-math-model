#[cfg(test)]
mod tests;

mod lin_expr_data;
mod lin_expr_sym;
mod lin_term_data;
mod lin_term_eval;
mod ops;
mod sum_data;

pub use lin_expr_data::{DLinExpr, ExprVec};
pub use lin_expr_sym::LinExpr;
pub use lin_term_data::{Coef, DLinTerm};
