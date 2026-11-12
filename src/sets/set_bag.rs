use crate::sets::{SetSlot, SetTyp};
use crate::{new_bag, Bag};
use orx_imp_vec::Elem;

pub struct SetBag<'d> {
    bag: Bag<SetSlot<'d>>,
}

impl Default for SetBag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> SetBag<'d> {
    pub fn push<'m>(&'m self, data: SetSlot<'d>) -> Elem<'m, SetSlot<'d>> {
        self.bag.push(data)
    }
}
