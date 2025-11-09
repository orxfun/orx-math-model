#[cfg(test)]
mod tests;

mod indices;
mod set_and_data;
mod set_data_collection;
mod set_data_dim0;
mod set_data_dim1;
mod set_gen;

pub use set_and_data::SetAndData;
pub use set_data_dim0::FunSetAndDataD0;
pub use set_data_dim1::FunSetAndDataD1;
pub use set_gen::SetGen;
