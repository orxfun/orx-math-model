use crate::vals::{DVal, FunVal};
use crate::{new_bag, Bag, SetTyp, Slot};
use orx_imp_vec::Elem;

pub struct ValBag<'d> {
    bag: Bag<DVal<'d>>,
}

impl Default for ValBag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> ValBag<'d> {
    pub fn push<'m>(&'m self, fun: Slot<'d, FunVal<'d>>) -> Elem<'m, DVal<'d>> {
        self.bag.push(DVal::new(fun))
    }
}
