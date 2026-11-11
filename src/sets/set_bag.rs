use crate::sets::{SetSlot, SetType};
use crate::{new_bag, Bag};

pub struct SetBag<'d> {
    bag: Bag<SetSlot<'d>>,
}

impl Default for SetBag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> SetBag<'d> {
    pub fn push(&self) -> char {
        todo!()
    }
}
