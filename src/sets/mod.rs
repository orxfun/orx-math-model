#[cfg(test)]
mod tests;

mod meta_set;
mod set_bag;
mod set_data;
mod set_fun;
mod set_fun_bag;
mod set_symbols;

pub use meta_set::{MetaCatInt, MetaCatStr, MetaOrd, MetaSet, SetTyp};
pub use set_bag::SetBag;
pub use set_data::DSet;
pub use set_fun::{FSet, FunSet, UFunSet};
pub use set_fun_bag::SetFunBag;
pub use set_symbols::{SetCatInt, SetCatStr, SetOrd};
