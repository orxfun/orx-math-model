use crate::{FSet, FunSlot, MetaSet, Res, SetTyp};

pub struct DSet<'d> {
    typ: SetTyp,
    fun: FunSlot<'d>,
}

impl<'d> DSet<'d> {
    pub fn bind<I0: MetaSet>(&self, fun: FSet<'d, I0>) -> Res {
        self.fun.bind_set::<I0>(fun);
        Ok(())
    }
}
