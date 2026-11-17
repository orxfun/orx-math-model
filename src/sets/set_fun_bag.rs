use crate::{sets::FunSet, Bag, SetTyp};
use orx_imp_vec::Elem;

pub struct SetFunBag<'d> {
    set: Bag<FunSet<'d>>,
}

impl<'d> SetFunBag<'d> {
    pub fn push<'m>(&'m self, typ: SetTyp) -> Elem<'m, FunSet<'d>> {
        self.set.push(FunSet::empty(typ))
    }
}
