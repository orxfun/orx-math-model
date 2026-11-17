use crate::vars::DVars;
use crate::{new_bag, Bag};
use orx_imp_vec::Elem;

pub struct VarBag<'d> {
    bag: Bag<DVars<'d>>,
}

impl Default for VarBag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> VarBag<'d> {
    pub fn push<'m>(&'m self, data: DVars<'d>) -> Elem<'m, DVars<'d>> {
        self.bag.push(data)
    }
}
