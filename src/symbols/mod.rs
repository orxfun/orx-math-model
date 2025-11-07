#[cfg(test)]
mod tests;

mod dep_set_indices;
pub(crate) mod pars;
pub(crate) mod sets;
mod symbol;
mod symbol_data_collection;
mod symbol_map;
mod symbol_meta;
mod symbol_ref;
mod symbol_ref_core;

pub use dep_set_indices::DependentSetIndices;
pub use pars::{par, Par};
pub use sets::{set_of, Set};
pub use symbol::Symbol;
pub use symbol_data_collection::SymbolDataCollection;
pub use symbol_map::SymbolMap;
pub use symbol_ref::SymbolRef;
