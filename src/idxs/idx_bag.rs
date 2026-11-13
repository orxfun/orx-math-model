use crate::idxs::idx_data::DIdx;
use crate::idxs::IdxSlot;
use crate::sets::{DSet, SetTyp};
use crate::{new_bag, Bag, Ptr};
use orx_imp_vec::Elem;

// collection

pub struct IdxBag<'d> {
    bag: Bag<DIdx<'d>>,
}

impl Default for IdxBag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> IdxBag<'d> {
    pub fn push<'m>(&'m self, set: Ptr<'d, DSet<'d>>, idx: IdxSlot<'d>) -> Elem<'m, DIdx<'d>> {
        let data = DIdx::new(set, idx);
        self.bag.push(data)
    }
}
