mod meta_sca;
mod sca_bag;
mod sca_data;
mod val;
mod var;

pub use meta_sca::{Sca, ScaTyp, ScaVar};
pub use sca_bag::ScaBag;
pub use sca_data::DSca;
pub use val::{DVal, FVal, Int, Num, ScaInt, ScaNum, Val, ValTyp};
pub use var::{DVar, FVar1, FVar2, Var};
