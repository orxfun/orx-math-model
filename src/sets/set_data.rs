use crate::sets::{FSet, FunSet};
use crate::Elements;
use crate::{Ptr, Res, ResValid, Set, SetTyp, Slot};

#[derive(derive_new::new)]
pub struct DSet<'d> {
    slot: Slot<'d, FunSet<'d>>,
}

impl<'d> DSet<'d> {
    pub fn typ(&self) -> SetTyp {
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
        debug_assert_eq!(S::typ(), self.typ());
        let fun = self.slot.get().get::<S>();
        fun(elements);
    }

    pub fn fun_set(&self) -> Ptr<'d, FunSet<'d>> {
        self.slot.ptr()
    }
}
