#[cfg(test)]
mod tests;

mod free_functions;
mod r#ref;
mod set_data;
mod set_meta;
mod set_tuples;
mod subsets;

pub use free_functions::set_of;
pub use r#ref::{Set, SetCore};
pub use set_data::SetData;
pub use set_meta::SetMeta;
pub use set_tuples::SetCollection;
