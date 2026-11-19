#[cfg(test)]
mod tests;

mod con;
mod ineq_op;
mod into_lin_expr;
mod sum;

pub use con::{con, for1, for2};
pub use ineq_op::{eq, geq, leq, IneqOp};
pub use into_lin_expr::IneqSide;
pub use sum::{over, sum, SumOver};
