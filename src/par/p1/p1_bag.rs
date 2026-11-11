use crate::idxs::IdxSlot;
use crate::par::p1::P1Slot;
use crate::sets::{SetSlot, SetType};
use crate::vals::Val;
use crate::{new_bag, Bag};
use orx_imp_vec::Elem;

pub struct DP1<'d, T: Val> {
    pub set: SetSlot<'d>,
    pub par: P1Slot<'d, T>,
}

// collection

pub struct P1Bag<'d> {
    int: Bag<DP1<'d, i64>>,
    num: Bag<DP1<'d, f64>>,
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
        par: P1Slot<'d, i64>,
    ) -> Elem<'m, DP1<'d, i64>> {
        self.int.push(DP1 { set, par })
    }

    pub fn push_num<'m>(
        &'m self,
        set: SetSlot<'d>,
        par: P1Slot<'d, f64>,
    ) -> Elem<'m, DP1<'d, f64>> {
        self.num.push(DP1 { set, par })
    }
}
