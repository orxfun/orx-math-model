use crate::sets::{FSet, FunSet};
use crate::{Elements, Res, ResValid, Set, SetTyp, Slot};
use core::mem::transmute;

#[derive(derive_new::new)]
pub struct DSet<'d> {
    slot: Slot<'d, FunSet<'d>>,
}

impl<'d> DSet<'d> {
    fn typ(&self) -> SetTyp {
        self.slot.get().typ
    }

    pub fn bind<S: Set>(&self, fun: FSet<'d, S>) -> Res {
        Res::dbg_assert_same_set(S::typ(), self.typ());
        let fun = FunSet::new::<S>(fun);
        self.slot.bind(fun)
    }

    #[cfg(test)]
    pub fn t_fill<S: Set>(&self, elements: &mut Elements<S::IdxRaw<'d>>) {
        Res::assert_slot_bound(self.slot.state()).unwrap();
        Res::assert_same_set_typ(S::typ(), self.typ()).unwrap();
        let fun = self.slot.get().get::<S>();
        fun(elements);
    }
}
