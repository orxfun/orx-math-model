use crate::{new_bag, sets::FunSet, Bag, MetaSet, Model, SetTyp};
use alloc::boxed::Box;
use core::mem::transmute;
use orx_imp_vec::Elem;

pub struct SetFunBag<'d> {
    set: Bag<FunSet<'d>>,
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
