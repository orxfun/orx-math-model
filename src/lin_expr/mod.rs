#[cfg(test)]
mod tests;

mod lin_expr_data;
mod lin_expr_sym;
mod lin_term_data;
mod ops;
mod sum_data;

pub use lin_expr_data::{DLinExpr, SumVec, TermVec};
pub use lin_expr_sym::LinExpr;
pub use lin_term_data::{Coef, DLinTerm};
pub use sum_data::DSum;
