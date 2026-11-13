use crate::{idxs::IdxVal, new_bag, Bag, Set, Model, SetTyp};
use alloc::boxed::Box;
use core::mem::transmute;
use orx_imp_vec::Elem;

pub struct IdxValBag<'d> {
    set: Bag<IdxVal<'d>>,
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
