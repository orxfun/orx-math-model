#[cfg(test)]
mod tests;

mod common_numbers;
mod meta_val;
mod num_data;
mod num_fun;
mod ops;
mod val_symbols;

pub use common_numbers::CommonNumbers;
pub use meta_val::{round_f64, AsF64, AsI64, Val, ValInt, ValNum, ValTyp};
pub use num_data::DNum;
pub use num_fun::{FInt, FNum};
pub use val_symbols::{Int, Num};
