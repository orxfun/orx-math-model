use crate::pars::p2::{FunP2, FP2};
use crate::sets::DSet;
use crate::{Elements, Ptr, Res, ResValid, Set, SetTyp, Slot, Val};

#[derive(derive_new::new)]
pub struct DP2<'d> {
    s0: Ptr<'d, DSet<'d>>,
    slot: Slot<'d, FunP2<'d>>,
}

impl<'d> DP2<'d> {
    pub fn bind<V: Val, I0: Set, I1: Set>(&self, fun: FP2<'d, V, I0, I1>) -> Res {
        Res::dbg_assert_same_val(V::typ(), self.slot.get().val_typ);
        Res::dbg_assert_same_set(I0::typ(), self.slot.get().i0_typ);
        let fun = FunP2::create::<V, I0, I1>(fun);
        self.slot.bind(fun)
    }

    pub fn at<V: Val, I0: Set, I1: Set>(&self, i0: I0::IdxRaw<'d>, i1: I1::IdxRaw<'d>) -> V::Raw {
        Res::dbg_assert_slot_bound(self.slot.state());
        Res::dbg_assert_same_val(V::typ(), self.slot.get().val_typ);
        Res::dbg_assert_same_set(I0::typ(), self.slot.get().i0_typ);
        let fun = self.slot.get().get::<V, I0, I1>();
        fun(i0, i1)
    }
}
