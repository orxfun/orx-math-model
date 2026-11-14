use crate::pars::p1::{FunP1, FP1};
use crate::sets::DSet;
use crate::val_idx_pairs::D1ValIdxPair;
use crate::{Elements, Ptr, Res, ResValid, Set, SetTyp, Slot, Val};

#[derive(derive_new::new)]
pub struct DP1<'d> {
    s0: Ptr<'d, DSet<'d>>,
    slot: Slot<'d, FunP1<'d>>,
}

impl<'d> DP1<'d> {
    pub fn bind<V: Val, I0: Set>(&self, fun: FP1<'d, V, I0>) -> Res
    where
        (V, I0): D1ValIdxPair<V = V, I0 = I0>,
    {
        Res::dbg_assert_same_val(V::typ(), self.slot.get().val_typ);
        Res::dbg_assert_same_set(I0::typ(), self.slot.get().i0_typ);
        let fun = FunP1::create::<V, I0>(fun);
        self.slot.bind(fun)
    }

    pub fn at<V: Val, I0: Set>(&self, i0: I0::IdxRaw<'d>) -> V::Raw
    where
        (V, I0): D1ValIdxPair<V = V, I0 = I0>,
    {
        Res::dbg_assert_slot_bound(self.slot.state());
        Res::dbg_assert_same_val(V::typ(), self.slot.get().val_typ);
        Res::dbg_assert_same_set(I0::typ(), self.slot.get().i0_typ);
        let fun = self.slot.get().get::<V, I0>();
        fun(i0)
    }
}
