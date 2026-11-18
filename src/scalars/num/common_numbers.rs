use crate::scalars::DNum;
use crate::{Bag, Int, Model, Ptr, ValTyp};
use alloc::boxed::Box;

pub struct CommonNumbers<'d> {
    pub one: Ptr<'d, DNum<'d>>,
    pub minus_one: Ptr<'d, DNum<'d>>,
    pub zero: Ptr<'d, DNum<'d>>,
}

impl<'d> CommonNumbers<'d> {
    pub fn new(num_bag: &Bag<DNum<'d>>) -> Self {
        let ptr = |data: DNum<'d>| Ptr::new_from_el(num_bag.push(data));

        Self {
            one: ptr(DNum::new(ValTyp::Int, Box::new(|| 1.0))),
            minus_one: ptr(DNum::new(ValTyp::Int, Box::new(|| -1.0))),
            zero: ptr(DNum::new(ValTyp::Int, Box::new(|| 0.0))),
        }
    }
}
