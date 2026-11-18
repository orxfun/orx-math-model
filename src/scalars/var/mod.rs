#[cfg(test)]
mod tests;

mod fun_var_to_idx;
mod fun_var_to_str;
mod var_data;
mod var_data_old;
mod var_fun;
mod var_fun1;
mod var_fun2;
mod var_sym;

pub use fun_var_to_idx::{FVarIdx, FVarIdxCtor, IdxEvalVec};
pub use fun_var_to_str::{FVarWrite, FVarWriteCtor};
pub use var_data_old::DVar;
pub use var_fun::UFunVar;
pub use var_fun1::{FVarIdx1, FunVar1, UFunV1};
pub use var_fun2::{FVarIdx2, FunVar2, UFunV2, UFunV2_I0};
pub use var_sym::Var;
