use crate::{new_bag, Bag};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// data

#[derive(Debug, PartialEq)]
pub enum ValValue {
    Int(i64),
    Num(f64),
}

pub enum ValFun<'d> {
    Int(Box<dyn Fn() -> i64 + 'd>),
    Num(Box<dyn Fn() -> f64 + 'd>),
}

impl<'d> ValFun<'d> {
    #[cfg(test)]
    pub fn test_eval(&self) -> ValValue {
        match self {
            Self::Int(f) => ValValue::Int(f()),
            Self::Num(f) => ValValue::Num(f()),
        }
    }
}

// bag

pub struct ValBag<'d> {
    bag: Bag<ValFun<'d>>,
}

impl Default for ValBag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> ValBag<'d> {
    pub fn push<'m>(&'m self, data: ValFun<'d>) -> Elem<'m, ValFun<'d>> {
        self.bag.push(data)
    }
}
