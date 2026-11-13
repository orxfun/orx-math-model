use crate::{Elements, Fun, FunSlot, MetaSet, Res, SetTyp, Validation};

#[derive(derive_new::new)]
pub struct DSet<'d> {
    typ: SetTyp,
    fun: FunSlot<'d>,
}

impl<'d> DSet<'d> {
    pub fn bind<S: MetaSet>(&self, fun: Fun<'d, S>) -> Res {
        Res::dbg_assert_same_set::<S>(self.typ);
        self.fun.bind_set::<S>(fun)
    }

    pub fn fill<S: MetaSet>(&self, elements: &mut Elements<S::IdxRaw<'d>>) -> Res {
        Res::assert_slot_bound(self.fun.state())?;
        Res::dbg_assert_same_set::<S>(self.typ);
        let fun = self.fun.get_set::<S>();
        fun(elements);
        Ok(())
    }
}
