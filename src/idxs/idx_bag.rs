use crate::idxs::{DIdx, IdxVal};
use crate::sets::DSet;
use crate::{new_bag, Bag, Ptr, SetTyp, Slot};
use orx_imp_vec::Elem;

pub struct IdxBag<'d> {
    bag: Bag<DIdx<'d>>,
}

impl Default for IdxBag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> IdxBag<'d> {
    pub fn push<'m>(
        &'m self,
        set_ptr: Ptr<'d, DSet<'d>>,
        slot: Slot<'d, IdxVal<'d>>,
    ) -> Elem<'m, DIdx<'d>> {
        self.bag.push(DIdx::new(set_ptr, slot))
    }
}
