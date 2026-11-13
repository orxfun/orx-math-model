use crate::fun::FunSlot;
use crate::{new_bag, Bag, MetaSet, Model, Slot};
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
    pub fn push<'m>(&'m self, m: &'m Model<'d>) -> FunSlot<'d> {
        let el = self.bag.push(Box::new(|| {}));
        FunSlot::new(Slot::new(m, el))
    }
}
