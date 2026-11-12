#[cfg(test)]
use crate::vals::ValVal;
use crate::vals::{Int, Num};
use crate::{new_bag, Bag, MetaVal, Val, ValSymSum};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// data

pub enum ValFun<'d> {
    Int(Box<dyn Fn() -> i64 + 'd>),
    Num(Box<dyn Fn() -> f64 + 'd>),
}

impl<'d> ValFun<'d> {
    #[cfg(test)]
    pub fn test_eval(&self) -> ValVal {
        match self {
            Self::Int(f) => ValVal::Int(f()),
            Self::Num(f) => ValVal::Num(f()),
        }
    }

    pub fn fun_ptr<V: MetaVal>(&self) -> &Box<dyn Fn() -> V::Val + 'd> {
        V::unpack_val_fun(self)
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
