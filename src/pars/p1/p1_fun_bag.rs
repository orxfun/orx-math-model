use crate::{new_bag, sets::FunSet, Bag, Model, Set, SetTyp};
use alloc::boxed::Box;
use core::mem::transmute;
use orx_imp_vec::Elem;

pub struct P1FunBag<'d> {
    set: Bag<FunSet<'d>>,
}

impl Default for P1FunBag<'_> {
    fn default() -> Self {
        Self { set: new_bag() }
    }
}

impl<'d> P1FunBag<'d> {
    pub fn push<'m>(&'m self, typ: SetTyp) -> Elem<'m, FunSet<'d>> {
        self.set.push(FunSet::empty(typ))
    }
}
