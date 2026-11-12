use crate::{ptr::RefPtr, values::ValVariant, Model};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// data

pub enum DVal<'d> {
    Int(Box<dyn Fn() -> i64 + 'd>),
    Num(Box<dyn Fn() -> f64 + 'd>),
}

impl<'d> DVal<'d> {
    pub fn new_int() -> Self {
        Self::Int(Box::new(|| 0))
    }

    pub fn new_num() -> Self {
        Self::Num(Box::new(|| 0.0))
    }

    pub fn value_as_variant(&self) -> ValVariant {
        match self {
            Self::Int(x) => ValVariant::Int(x()),
            Self::Num(x) => ValVariant::Num(x()),
        }
    }
}
