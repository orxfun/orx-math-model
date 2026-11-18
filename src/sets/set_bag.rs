use crate::sets::{DSet, FunSet};
use crate::{Bag, Slot};
use orx_imp_vec::Elem;

pub struct SetBag<'d> {
    bag: Bag<DSet<'d>>,
}

impl<'d> SetBag<'d> {
    pub fn push<'m>(&'m self, fun: Slot<'d, FunSet<'d>>) -> Elem<'m, DSet<'d>> {
        self.bag.push(DSet::new(fun))
    }
}
