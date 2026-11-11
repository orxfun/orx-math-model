#[cfg(test)]
mod tests;

mod set_bag;
mod set_slots;
mod symbols;

pub use set_bag::SetBag;
pub use set_slots::{SetFun, SetSlot, SetSlots, SetType};
pub use symbols::{CatSetInt, CatSetStr, OrdSet};
