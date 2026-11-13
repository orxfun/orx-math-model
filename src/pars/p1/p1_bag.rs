use crate::pars::p1::{FunP1, DP1};
use crate::sets::DSet;
use crate::{new_bag, Bag, Ptr, SetTyp, Slot};
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
    pub fn push<'m>(
        &'m self,
        s0_ptr: Ptr<'d, DSet<'d>>,
        fun: Slot<'d, FunP1<'d>>,
    ) -> Elem<'m, DP1<'d>> {
        self.bag.push(DP1::new(s0_ptr, fun))
    }
}
