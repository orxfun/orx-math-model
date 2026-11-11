#[cfg(test)]
mod tests;

mod val_bag;
mod val_symbols;
mod val_trait;

pub use val_bag::{ValBag, ValFun, ValVal};
pub use val_symbols::{Int, Num};
pub use val_trait::Val;
