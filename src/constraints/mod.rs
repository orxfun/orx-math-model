#[cfg(test)]
mod tests;

mod c1_fun;
mod con_data;
mod con_macro;
mod con_sym;

pub use con_data::{DCon, IneqTyp};
pub use con_sym::Constraint;
