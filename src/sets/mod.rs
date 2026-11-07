#[cfg(test)]
mod tests;

mod cat;
mod ord;
mod set;

pub use cat::CatSet;
pub use ord::OrdSet;
pub use set::{DSet, Set};
