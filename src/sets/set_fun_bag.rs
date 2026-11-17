use crate::{new_bag, sets::FunSet, BagKind, SetTyp};
use orx_imp_vec::Elem;

pub struct SetFunBag<'d> {
    set: BagKind<FunSet<'d>>,
}

impl Default for SetFunBag<'_> {
    fn default() -> Self {
        Self { set: new_bag() }
    }
}

impl<'d> SetFunBag<'d> {
    pub fn push<'m>(&'m self, typ: SetTyp) -> Elem<'m, FunSet<'d>> {
        self.set.push(FunSet::empty(typ))
    }
}
