use crate::{Elements, FSet, FunSet, MetaSet, Res, SetTyp, Slot, Validation};
use core::mem::transmute;

#[derive(derive_new::new)]
pub struct DSet<'d> {
    typ: SetTyp,
    fun: Slot<'d, FunSet<'d>>,
}

impl<'d> DSet<'d> {
    pub fn bind<S: MetaSet>(&self, fun: FSet<'d, S>) -> Res {
        Res::dbg_assert_same_set(S::set_typ(), self.typ);
        let fun = match S::set_typ() {
            SetTyp::CatInt => FunSet::CatInt(unsafe { transmute(fun) }),
            SetTyp::CatStr => FunSet::CatStr(unsafe { transmute(fun) }),
            SetTyp::Ord => FunSet::Ord(unsafe { transmute(fun) }),
        };
        self.fun.bind(fun)
    }

    pub fn fill<S: MetaSet>(&self, elements: &mut Elements<S::IdxRaw<'d>>) -> Res {
        Res::assert_slot_bound(self.fun.state())?;
        Res::dbg_assert_same_set(S::set_typ(), self.typ);
        let fun = self.fun.get().get::<S>();
        fun(elements);
        Ok(())
    }
}
