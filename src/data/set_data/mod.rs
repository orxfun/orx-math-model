#[cfg(test)]
mod tests;

mod empty;
mod fun_set_data;
mod indices;
mod set_and_data;
mod set_data_collection;

pub use empty::EmptySet;
pub use fun_set_data::FunSetAndData;
pub use indices::IndexValuesIter;
pub use set_and_data::{SetData, SetDataCore};
pub use set_data_collection::SetDataCollection;
