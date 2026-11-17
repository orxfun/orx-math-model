#[cfg(test)]
mod tests;

mod var_bag;
mod var_data;
mod var_symbols;
mod var_typ;

pub use var_bag::VarBag;
pub use var_data::DVars;
pub use var_symbols::{VarD1, VarD2};
pub use var_typ::{Bounds, VarType};
