pub mod v1;
pub mod v2;
mod var_bag;
mod var_data;
mod var_typ;

pub use var_bag::VarBag;
pub use var_data::DV;
pub use var_typ::{Bounds, VarType};
