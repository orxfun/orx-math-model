#[cfg(test)]
mod tests;

mod free_functions;
mod indep_set_collection;
mod r#ref;
mod set_collection;
mod set_data;
mod set_meta;
mod subsets;

pub use free_functions::set_of;
pub use indep_set_collection::IndependentSetCollection;
pub use r#ref::{Set, SetCore};
pub use set_collection::SetCollection;
pub use set_data::SetData;
pub use set_meta::SetMeta;
