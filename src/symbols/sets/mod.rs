#[cfg(test)]
mod tests;

mod dep_set_indices;
mod free_functions;
// mod indices; // TODO: park for now until we fill sets with data
mod r#ref;
mod set_data;
mod set_meta;
mod subsets;

pub use dep_set_indices::DependentSetIndices;
pub use free_functions::set_of;
pub use r#ref::{Set, SetCore};
pub use set_data::SetData;
pub use set_meta::SetMeta;
