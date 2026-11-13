#[cfg(test)]
mod tests;

mod meta_set;
mod meta_set_sym;
mod set_bag;
mod set_data;
mod set_fun;
mod set_fun_bag;
mod set_symbols;

pub use meta_set::{Set, SetCatInt, SetCatStr, SetOrd, SetTyp};
pub use meta_set_sym::SetSym;
pub use set_bag::SetBag;
pub use set_data::DSet;
pub use set_fun::{FSet, FunSet, UFunSet};
pub use set_fun_bag::SetFunBag;
pub use set_symbols::{CatInt, CatStr, Ord};
