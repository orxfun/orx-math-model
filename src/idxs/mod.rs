#[cfg(test)]
mod tests;

mod idx_bag;
mod idx_slots;
mod idx_symbols;
mod idx_trait;

pub use idx_bag::IdxBag;
pub use idx_slots::{IdxSlot, IdxSlots, IdxVal};
pub use idx_symbols::{CatIdxInt, CatIdxStr, OrdIdx};
pub use idx_trait::Idx;
