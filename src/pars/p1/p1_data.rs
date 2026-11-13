use crate::pars::p1::{FunP1, FP1};
use crate::sets::DSet;
use crate::{Elements, Ptr, Res, ResValid, Set, SetTyp, Slot, Val};
use core::mem::transmute;

#[derive(derive_new::new)]
pub struct DP1<'d> {
    s0: Ptr<'d, DSet<'d>>,
    slot: Slot<'d, FunP1<'d>>,
}

impl<'d> DP1<'d> {
    pub fn bind<V: Val, I0: Set>(&self, fun: FP1<'d, V, I0>) -> Res {
        Res::dbg_assert_same_val(V::typ(), self.slot.get().val_typ);
        Res::dbg_assert_same_set(I0::typ(), self.slot.get().i0_typ);
        let fun = FunP1::new::<V, I0>(fun);
        self.slot.bind(fun)
    }

    #[cfg(test)]
    pub fn t_at<V: Val, I0: Set>(&self, i0: I0::IdxRaw<'d>) -> V::Raw
    where
        (V, I0): crate::val_idx_pairs::ValIdxPairD1<V = V, I0 = I0>,
    {
        Res::assert_slot_bound(self.slot.state()).unwrap();
        Res::dbg_assert_same_val(V::typ(), self.slot.get().val_typ);
        Res::dbg_assert_same_set(I0::typ(), self.slot.get().i0_typ);
        let fun = self.slot.get().get::<V, I0>();
        fun(i0)
    }
}
