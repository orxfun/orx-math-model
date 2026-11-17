use crate::{new_bag, scalars::num::DNum, Bag, Slot};
use orx_imp_vec::Elem;

pub struct SetBag<'d> {
    bag: Bag<DNum<'d>>,
}

impl Default for SetBag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> SetBag<'d> {
    pub fn push<'m>(&'m self, data: DNum<'d>) -> Elem<'m, DNum<'d>> {
        self.bag.push(data)
    }
}
