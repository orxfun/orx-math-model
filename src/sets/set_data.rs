use crate::sets::{FSet, FunSet};
use crate::{Elements, MetaSet, Res, SetTyp, Slot, ResValid};
use core::mem::transmute;

#[derive(derive_new::new)]
pub struct DSet<'d> {
    typ: SetTyp,
    fun: Slot<'d, FunSet<'d>>,
}

impl<'d> DSet<'d> {
    pub fn bind<S: MetaSet>(&self, fun: FSet<'d, S>) -> Res {
        Res::dbg_assert_same_set(S::typ(), self.typ);
        let fun = FunSet::new::<S>(fun);
        self.fun.bind(fun)
    }

    #[cfg(test)]
    pub fn t_fill<S: MetaSet>(&self, elements: &mut Elements<S::IdxRaw<'d>>) {
        Res::assert_slot_bound(self.fun.state()).unwrap();
        Res::assert_same_set_typ(S::typ(), self.typ).unwrap();
        let fun = self.fun.get().get::<S>();
        fun(elements);
    }
}
