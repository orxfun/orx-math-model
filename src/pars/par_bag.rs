use crate::pars::{DPar, ParFun};
use crate::sets::DSet;
use crate::{new_bag, Bag, DimVec, Ptr, Slot};
use orx_imp_vec::Elem;

pub struct ParBag<'d> {
    bag: Bag<DPar<'d>>,
}

impl Default for ParBag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> ParBag<'d> {
    pub fn push<'m>(
        &'m self,
        sets: DimVec<Ptr<'d, DSet<'d>>>,
        fun: Slot<'d, ParFun<'d>>,
    ) -> Elem<'m, DPar<'d>> {
        let data = DPar::new(sets.dim(), sets, fun);
        self.bag.push(data)
    }
}
