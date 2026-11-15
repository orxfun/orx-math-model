mod meta_val;
mod val_data;
mod val_fun;
mod val_symbols;

pub use meta_val::{Val, ScaInt, ScaNum, ValTyp};
pub use val_data::DVal;
pub use val_fun::{FVal, UFunVal};
pub use val_symbols::{Int, Num};
