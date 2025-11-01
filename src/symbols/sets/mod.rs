#[cfg(test)]
mod tests;

mod dep_set_indices;
mod free_functions;
// mod indices; // TODO: park for now until we fill sets with data
mod set;
mod set_data;
mod set_meta;
mod subsets;

pub use free_functions::set_of;
pub use set::Set;
pub use set_data::SetData;
pub use set_meta::SetMeta;
