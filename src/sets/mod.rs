#[cfg(test)]
mod tests;

mod set_bag;
mod set_slots;
mod set_symbols;

pub use set_bag::SetBag;
pub use set_slots::{SetFun, SetSlot, SetSlots, SetType};
pub use set_symbols::{CatSetInt, CatSetStr, OrdSet};
