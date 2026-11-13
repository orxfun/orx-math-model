use crate::pars::p1::{FunP1, DP1};
use crate::{new_bag, Bag, SetTyp, Slot};
use orx_imp_vec::Elem;

pub struct P1Bag<'d> {
    bag: Bag<DP1<'d>>,
}

impl Default for P1Bag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> P1Bag<'d> {
    pub fn push<'m>(&'m self, fun: Slot<'d, FunP1<'d>>) -> Elem<'m, DP1<'d>> {
        self.bag.push(DP1::new(fun))
    }
}
