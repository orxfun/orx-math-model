#[cfg(test)]
mod tests;

mod meta_set;
mod set_bag;
mod set_slots;
mod set_symbols;

pub use meta_set::{MetaCatSetInt, MetaCatSetStr, MetaOrdSet, MetaSet, SetTyp};
pub use set_bag::SetBag;
pub use set_slots::{SetFun, SetSlot, SetSlots};
pub use set_symbols::{CatSetInt, CatSetStr, OrdSet};
