use crate::sets::{DSet, SetTyp};
use crate::{new_bag, Bag, FunSet, Slot};
use orx_imp_vec::Elem;

pub struct SetBag<'d> {
    bag: Bag<DSet<'d>>,
}

impl Default for SetBag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> SetBag<'d> {
    pub fn push<'m>(&'m self, typ: SetTyp, fun: Slot<'d, FunSet<'d>>) -> Elem<'m, DSet<'d>> {
        self.bag.push(DSet::new(typ, fun))
    }
}
