use super::bag::{new_bag, Bag};
use crate::numbers::ValFun;
use orx_imp_vec::Elem;

pub struct Numbers<'d> {
    int: Bag<ValFun<'d, i64>>,
    num: Bag<ValFun<'d, f64>>,
}

impl<'d> Numbers<'d> {
    pub fn new() -> Self {
        Self {
            int: new_bag(),
            num: new_bag(),
        }
    }

    pub fn int<'m>(&'m self) -> Elem<'m, ValFun<'d, i64>> {
        self.int.push(ValFun::empty())
    }

    pub fn num<'m>(&'m self) -> Elem<'m, ValFun<'d, f64>> {
        self.num.push(ValFun::empty())
    }
}
