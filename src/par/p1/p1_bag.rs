use crate::idxs::IdxSlot;
use crate::par::p1::P1Slot;
use crate::sets::{SetSlot, SetType};
use crate::vals::{MetaInt, MetaNum, MetaVal};
use crate::{new_bag, Bag};
use orx_imp_vec::Elem;

pub struct DP1<'d, V: MetaVal> {
    pub set: SetSlot<'d>,
    pub par: P1Slot<'d, V>,
}

// collection

pub struct P1Bag<'d> {
    int: Bag<DP1<'d, MetaInt>>,
    num: Bag<DP1<'d, MetaNum>>,
}

impl Default for P1Bag<'_> {
    fn default() -> Self {
        Self {
            int: new_bag(),
            num: new_bag(),
        }
    }
}

impl<'d> P1Bag<'d> {
    pub fn push_int<'m>(
        &'m self,
        set: SetSlot<'d>,
        par: P1Slot<'d, MetaInt>,
    ) -> Elem<'m, DP1<'d, MetaInt>> {
        self.int.push(DP1 { set, par })
    }

    pub fn push_num<'m>(
        &'m self,
        set: SetSlot<'d>,
        par: P1Slot<'d, MetaNum>,
    ) -> Elem<'m, DP1<'d, MetaNum>> {
        self.num.push(DP1 { set, par })
    }
}
