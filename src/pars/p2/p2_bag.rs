use crate::pars::p2::{FunP2, DP2};
use crate::sets::DSet;
use crate::{new_bag, Bag, Ptr, SetTyp, Slot};
use orx_imp_vec::Elem;

pub struct P2Bag<'d> {
    bag: Bag<DP2<'d>>,
}

impl Default for P2Bag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> P2Bag<'d> {
    pub fn push<'m>(
        &'m self,
        s0_ptr: Ptr<'d, DSet<'d>>,
        s1_ptr: Ptr<'d, DSet<'d>>,
        fun: Slot<'d, FunP2<'d>>,
    ) -> Elem<'m, DP2<'d>> {
        self.bag.push(DP2::new(s0_ptr, s1_ptr, fun))
    }
}
