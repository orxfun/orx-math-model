#[cfg(test)]
mod tests;

mod cat;
mod ord;
mod set;

pub use cat::{CatSet, CatSet2};
pub use ord::{OrdSet, OrdSet2};
pub use set::{DSet, DSet2, Set};
