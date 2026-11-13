use crate::{Elements, FSet, FunSlot, MetaSet, Res, SetTyp, Validation};

pub struct DSet<'d> {
    typ: SetTyp,
    fun: FunSlot<'d>,
}

impl<'d> DSet<'d> {
    pub fn bind<S: MetaSet>(&self, fun: FSet<'d, S>) -> Res {
        Res::dbg_assert_same_set::<S>(self.typ);
        self.fun.bind_set::<S>(fun)
    }

    pub fn fill<S: MetaSet>(&self, elements: &mut Elements<S::IdxRaw<'d>>) {
        Res::dbg_assert_same_set::<S>(self.typ);
        let fun = self.fun.get_set::<S>();
        fun(elements)
    }
}
