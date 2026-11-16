use crate::vars::DV;
use crate::{new_bag, Bag};
use orx_imp_vec::Elem;

pub struct VarBag<'d> {
    bag: Bag<DV<'d>>,
}

impl Default for VarBag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> VarBag<'d> {
    pub fn push<'m>(&'m self, data: DV<'d>) -> Elem<'m, DV<'d>> {
        self.bag.push(data)
    }
}
