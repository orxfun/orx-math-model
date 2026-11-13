use crate::{new_bag, pars::p1::FunP1, Bag, Model, Set, SetTyp, ValTyp};
use alloc::boxed::Box;
use core::mem::transmute;
use orx_imp_vec::Elem;

pub struct P1FunBag<'d> {
    set: Bag<FunP1<'d>>,
}

impl Default for P1FunBag<'_> {
    fn default() -> Self {
        Self { set: new_bag() }
    }
}

impl<'d> P1FunBag<'d> {
    pub fn push<'m>(&'m self, val_typ: ValTyp, i0_typ: SetTyp) -> Elem<'m, FunP1<'d>> {
        self.set.push(FunP1::empty(val_typ, i0_typ))
    }
}
