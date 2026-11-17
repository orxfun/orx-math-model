use crate::sets::{DSet, FunSet};
use crate::{new_bag, BagKind, Slot};
use orx_imp_vec::Elem;

pub struct SetBag<'d> {
    bag: BagKind<DSet<'d>>,
}

impl Default for SetBag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> SetBag<'d> {
    pub fn push<'m>(&'m self, fun: Slot<'d, FunSet<'d>>) -> Elem<'m, DSet<'d>> {
        self.bag.push(DSet::new(fun))
    }
}
