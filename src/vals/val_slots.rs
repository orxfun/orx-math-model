use crate::{new_bag, Bag, Elements, Res, Slot};
use alloc::{boxed::Box, string::String};

pub enum ValType {
    Int,
    Num,
}

pub enum ValFun<'d> {
    Int(Box<dyn Fn() -> i64 + 'd>),
    Num(Box<dyn Fn() -> f64 + 'd>),
}

#[derive(Clone)]
pub enum ValSlot<'d> {
    Int(Slot<'d, Box<dyn Fn() -> i64 + 'd>>),
    Num(Slot<'d, Box<dyn Fn() -> f64 + 'd>>),
}

impl<'d> ValSlot<'d> {
    pub fn bind(&self, fun: ValFun<'d>) -> Res {
        match (self, fun) {
            (Self::Int(s), ValFun::Int(f)) => s.bind(f),
            (Self::Num(s), ValFun::Num(f)) => s.bind(f),
            _ => unreachable!("TODO: why"),
        }
    }
}

// collection

pub struct ValSlots<'d> {
    int: Bag<Box<dyn Fn() -> i64 + 'd>>,
    num: Bag<Box<dyn Fn() -> f64 + 'd>>,
}

impl<'d> Default for ValSlots<'d> {
    fn default() -> Self {
        Self {
            int: new_bag(),
            num: new_bag(),
        }
    }
}

impl<'d> ValSlots<'d> {
    pub fn push(&self, t: ValType) -> ValSlot<'d> {
        match t {
            ValType::Int => {
                let el = self.int.push(Box::new(|| 0));
                ValSlot::Int(el.into())
            }
            ValType::Num => {
                let el = self.num.push(Box::new(|| 0.0));
                ValSlot::Num(el.into())
            }
        }
    }
}
