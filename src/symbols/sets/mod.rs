#[cfg(test)]
mod tests;

mod dependent;
mod independent;
mod indices;
mod set;
mod set_data;
mod set_gen;
mod set_symbol;
mod subsets;

pub use independent::IntoSet;
pub use set::Set;
pub use set_data::SetKind;
pub use set_symbol::SetSymbol;
