use crate::{new_bag, Bag, FunSet, MetaSet, Model, SetTyp};
use alloc::boxed::Box;
use core::mem::transmute;
use orx_imp_vec::Elem;

pub struct FunBag<'d> {
    set: Bag<FunSet<'d>>,
}

impl Default for FunBag<'_> {
    fn default() -> Self {
        Self { set: new_bag() }
    }
}

impl<'d> FunBag<'d> {
    pub fn push_set<'m>(&'m self, typ: SetTyp) -> Elem<'m, FunSet<'d>> {
        self.set.push(FunSet::new(typ))
    }
}
