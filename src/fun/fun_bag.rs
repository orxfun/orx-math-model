use crate::{new_bag, Bag};
use alloc::boxed::Box;

pub struct FunBag<'d> {
    bag: Bag<Box<dyn Fn() + 'd>>,
}

impl Default for FunBag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> FunBag<'d> {
    // pub fn
}
