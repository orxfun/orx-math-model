use crate::vars::DVars;
use crate::{new_bag, Bag};
use orx_imp_vec::Elem;

pub struct LinExpBag<'d> {
    bag: Bag<DVars<'d>>,
}

impl Default for LinExpBag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> LinExpBag<'d> {
    pub fn push<'m>(&'m self, data: DVars<'d>) -> Elem<'m, DVars<'d>> {
        self.bag.push(data)
    }
}
