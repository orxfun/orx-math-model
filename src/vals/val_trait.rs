use crate::vals::ValVal;

pub trait Val {
    fn as_val_val(self) -> ValVal;
}

// int

impl Val for i64 {
    fn as_val_val(self) -> ValVal {
        ValVal::Int(self)
    }
}

// num

impl Val for f64 {
    fn as_val_val(self) -> ValVal {
        ValVal::Num(self)
    }
}
