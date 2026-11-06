use crate::bag::{new_bag, Bag};
use alloc::boxed::Box;

pub struct Params<'d> {
    // d1-fun
    fun_u_num: Bag<Box<dyn Fn(usize) -> f64 + 'd>>,
    fun_u_int: Bag<Box<dyn Fn(usize) -> i64 + 'd>>,
    fun_s_num: Bag<Box<dyn Fn(&'d str) -> f64 + 'd>>,
    fun_s_int: Bag<Box<dyn Fn(&'d str) -> i64 + 'd>>,
}

impl<'d> Params<'d> {
    pub fn new() -> Self {
        Self {
            // d1-fun
            fun_u_num: new_bag(),
            fun_u_int: new_bag(),
            fun_s_num: new_bag(),
            fun_s_int: new_bag(),
        }
    }
}
