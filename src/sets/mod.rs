#[cfg(test)]
mod tests;

mod cat;
mod new_set;
mod ord;
mod set;

pub use cat::CatSet;
pub use new_set::DSet2;
pub use ord::OrdSet;
pub use set::{DSet, Set};
