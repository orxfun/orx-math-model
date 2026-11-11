use crate::idxs::IdxSlot;
use crate::sets::{SetSlot, SetType};
use crate::{new_bag, Bag};
use orx_imp_vec::Elem;

pub struct DIdx<'d> {
    set: SetSlot<'d>,
    idx: IdxSlot<'d>,
}

pub struct IdxBag<'d> {
    bag: Bag<DIdx<'d>>,
}

impl Default for IdxBag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> IdxBag<'d> {
    pub fn push<'m>(&'m self, set: SetSlot<'d>, idx: IdxSlot<'d>) -> Elem<'m, DIdx<'d>> {
        let data = DIdx { set, idx };
        self.bag.push(data)
    }
}
