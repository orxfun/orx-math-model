use crate::var::{FunV, FV1, FV2};
use crate::{new_bag, Bag, Model, Set, SetTyp, ValTyp};
use alloc::boxed::Box;
use core::mem::transmute;
use orx_imp_vec::Elem;

pub struct VarFunBag<'d> {
    set: Bag<FunV<'d>>,
}

impl Default for VarFunBag<'_> {
    fn default() -> Self {
        Self { set: new_bag() }
    }
}

impl<'d> VarFunBag<'d> {
    pub fn push_d1<'m, I0: Set>(&'m self, fun: FV1<'d, I0>) -> Elem<'m, FunV<'d>> {
        let fun = FunV::create_d1::<I0>(fun);
        self.set.push(fun)
    }

    pub fn push_d2<'m, I0: Set, I1: Set>(&'m self, fun: FV2<'d, I0, I1>) -> Elem<'m, FunV<'d>> {
        let fun = FunV::create_d2::<I0, I1>(fun);
        self.set.push(fun)
    }
}
