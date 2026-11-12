use crate::idxs::IdxSlot;
use crate::par::p1::P1Slot;
use crate::sets::SetSlot;
use crate::{new_bag, Bag, MetaInt, MetaNum, MetaVal, SetTyp, ValTyp};
#[cfg(test)]
use crate::{vals::ValVal, MetaSet, Res};
use core::mem::transmute;
use orx_imp_vec::Elem;

#[derive(derive_new::new)]
pub struct DP1<'d, V: MetaVal> {
    pub set: SetSlot<'d>,
    pub par: P1Slot<'d, V>,
}

impl<'d, V: MetaVal> DP1<'d, V> {
    #[cfg(test)]
    pub fn test_at<M: MetaSet>(&self, idx: M::IdxRaw<'d>) -> Res<ValVal> {
        self.par.test_at::<M>(idx)
    }
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
    pub fn push<'m, V: MetaVal>(
        &'m self,
        par: P1Slot<'d, V>,
        s0: SetSlot<'d>,
    ) -> Elem<'m, DP1<'d, V>> {
        match V::val_typ() {
            ValTyp::Int => unsafe { transmute(self.int.push(DP1::new(s0, transmute(par)))) },
            ValTyp::Num => unsafe { transmute(self.num.push(DP1::new(s0, transmute(par)))) },
        }
    }
}
