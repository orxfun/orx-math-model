#[cfg(test)]
mod tests;

mod val_bag;
mod val_symbols;

pub use val_bag::{ValBag, ValFun, ValValue};
pub use val_symbols::{Int, Num};
