use crate::{new_bag, vals::FunVal, Bag, Model, Set, SetTyp};
use alloc::boxed::Box;
use core::mem::transmute;
use orx_imp_vec::Elem;

pub struct ValFunBag<'d> {
    set: Bag<FunVal<'d>>,
}

impl Default for ValFunBag<'_> {
    fn default() -> Self {
        Self { set: new_bag() }
    }
}

impl<'d> ValFunBag<'d> {
    pub fn push<'m>(&'m self, fun: FunVal<'d>) -> Elem<'m, FunVal<'d>> {
        self.set.push(fun)
    }
}
