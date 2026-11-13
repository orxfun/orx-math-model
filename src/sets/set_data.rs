use crate::{FSet, FunSlot, MetaSet, Res, SetTyp, Validation};

pub struct DSet<'d> {
    typ: SetTyp,
    fun: FunSlot<'d>,
}

impl<'d> DSet<'d> {
    pub fn bind<I0: MetaSet>(&self, fun: FSet<'d, I0>) -> Res {
        Res::dbg_assert_same_set::<I0>(self.typ);
        self.fun.bind_set::<I0>(fun)
    }
}
