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
}

// bag

pub struct ValBag<'d> {
    bag: Bag<ValFun<'d>>,
    sym_bag: Bag<ValSymSum<'d, 'd>>,
}

impl Default for ValBag<'_> {
    fn default() -> Self {
        Self {
            bag: new_bag(),
            sym_bag: new_bag(),
        }
    }
}

impl<'d> ValBag<'d> {
    pub fn push<'m>(&'m self, data: ValFun<'d>) -> Elem<'m, ValFun<'d>> {
        self.bag.push(data)
    }

    pub fn push_sym<'m>(&'m self, sym: impl Val<'m, 'd>) -> Elem<'m, ValSymSum<'d, 'd>> {
        let sym: ValSymSum<'d, 'd> = match sym.into_sym_sum() {
            ValSymSum::Int(x) => ValSymSum::Int(unsafe { x.create_sym_ref() }.into()),
            ValSymSum::Num(x) => ValSymSum::Num(unsafe { x.create_sym_ref() }.into()),
        };
        self.sym_bag.push(sym)
    }
}
