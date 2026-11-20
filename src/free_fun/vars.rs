use crate::VarType;
use alloc::string::{String, ToString};

pub fn var(key: impl ToString) -> BVar {
    BVar(key.to_string(), VarType::Continuous)
}

pub struct BVar(String, VarType);

impl BVar {}
