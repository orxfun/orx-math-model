use crate::{new_bag, pars::p2::FunP2, Bag, Model, Set, SetTyp, ValTyp};
use alloc::boxed::Box;
use core::mem::transmute;
use orx_imp_vec::Elem;

pub struct P2FunBag<'d> {
    set: Bag<FunP2<'d>>,
}

impl Default for P2FunBag<'_> {
    fn default() -> Self {
        Self { set: new_bag() }
    }
}

impl<'d> P2FunBag<'d> {
    pub fn push<'m>(
        &'m self,
        val_typ: ValTyp,
        i0_typ: SetTyp,
        i1_typ: SetTyp,
    ) -> Elem<'m, FunP2<'d>> {
        self.set.push(FunP2::empty(val_typ, i0_typ, i1_typ))
    }
}
