use crate::{fun::FunSlot, new_bag, Bag, Model, Slot};
use alloc::boxed::Box;
use core::mem::transmute;

pub struct FunBag<'d> {
    bag: Bag<Box<dyn Fn() + 'd>>,
}

impl Default for FunBag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> FunBag<'d> {
    pub fn push<'m, F>(&'m self, m: &'m Model<'d>) -> FunSlot<'d> {
        let el = self.bag.push(Box::new(|| {}));
        FunSlot::new(Slot::new(m, el))
    }
}
