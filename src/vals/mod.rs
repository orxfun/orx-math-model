#[cfg(test)]
mod tests;

mod val_bag;
mod val_slots;
mod val_symbols;

pub use val_bag::ValBag;
pub use val_slots::{ValFun, ValSlot, ValSlots, ValType};
pub use val_symbols::{Int, Num};
