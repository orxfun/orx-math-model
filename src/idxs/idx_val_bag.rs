use crate::{idxs::IdxVal, new_bag, BagKind, SetTyp};
use orx_imp_vec::Elem;

pub struct IdxValBag<'d> {
    set: BagKind<IdxVal<'d>>,
}

impl Default for IdxValBag<'_> {
    fn default() -> Self {
        Self { set: new_bag() }
    }
}

impl<'d> IdxValBag<'d> {
    pub fn push<'m>(&'m self, typ: SetTyp) -> Elem<'m, IdxVal<'d>> {
        self.set.push(IdxVal::empty(typ))
    }
}
