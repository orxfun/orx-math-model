use crate::var::DV;
use crate::{new_bag, Bag, Model, Set, SetTyp, ValTyp};
use alloc::boxed::Box;
use core::mem::transmute;
use orx_imp_vec::Elem;

pub struct VarFunBag<'d> {
    set: Bag<DV<'d>>,
}

impl Default for VarFunBag<'_> {
    fn default() -> Self {
        Self { set: new_bag() }
    }
}

impl<'d> VarFunBag<'d> {
    pub fn push<'m>(&'m self, fun: DV<'d>) -> Elem<'m, DV<'d>> {
        self.set.push(fun)
    }
}
