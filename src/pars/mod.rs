#[cfg(test)]
mod tests;

pub mod p1;
pub mod p2;

mod par_bag;
mod par_data;
mod par_fun;
mod par_fun_bag;
mod par_symbols;

pub use par_bag::ParBag;
pub use par_data::DPar;
pub use par_fun::ParFun;
pub use par_fun_bag::ParFunBag;
pub use par_symbols::{ParD1, ParD2};
