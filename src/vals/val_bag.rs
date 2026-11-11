use crate::vals::{ValSlot, ValType};
use crate::{new_bag, Bag};
use orx_imp_vec::Elem;

pub struct ValBag<'d> {
    bag: Bag<ValSlot<'d>>,
}

impl Default for ValBag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> ValBag<'d> {
    pub fn push<'m>(&'m self, data: ValSlot<'d>) -> Elem<'m, ValSlot<'d>> {
        self.bag.push(data)
    }
}
