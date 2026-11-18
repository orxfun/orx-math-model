#[cfg(test)]
mod tests;

mod var_data;
mod var_symbols;
mod var_typ;

pub use var_data::DV;
pub use var_symbols::{VarD1, VarD2};
pub use var_typ::{Bounds, VarType};
