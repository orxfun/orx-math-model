use crate::VarTyp;
use alloc::string::{String, ToString};

pub fn var(key: impl ToString) -> BVar {
    BVar(key.to_string(), VarTyp::Continuous)
}

pub struct BVar(String, VarTyp);

impl BVar {}
