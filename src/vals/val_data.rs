use crate::vals::{FVal, FunVal};
use crate::{Elements, Res, ResValid, Slot, Val, ValTyp};
use core::mem::transmute;

#[derive(derive_new::new)]
pub struct DVal<'d> {
    slot: Slot<'d, FunVal<'d>>,
}

impl<'d> DVal<'d> {
    fn typ(&self) -> ValTyp {
        self.slot.get().typ
    }

    pub fn bind<V: Val>(&self, fun: FVal<'d, V>) -> Res {
        Res::dbg_assert_same_val(V::typ(), self.typ());
        let fun = FunVal::new::<V>(fun);
        self.slot.bind(fun)
    }

    pub fn eval<V: Val>(&self) -> V::Raw {
        Res::dbg_assert_slot_bound(self.slot.state());
        let fun = self.slot.get().get::<V>();
        fun()
    }
}
