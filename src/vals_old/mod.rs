#[cfg(test)]
mod tests;

mod meta_val;
mod val_bag;
mod val_symbols;

pub use meta_val::{MetaInt, MetaNum, MetaVal, Val, ValSymSum, ValTyp, ValVal};
pub use val_bag::{ValBag, ValFun};
pub use val_symbols::{Int, Num};
