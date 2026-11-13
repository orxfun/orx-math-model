use crate::{new_bag, vals::FunVal, Bag, Model, Set, SetTyp, Val};
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
    pub fn push<'m, V: Val>(&'m self, fun: impl Fn() -> V::Raw + 'd) -> Elem<'m, FunVal<'d>> {
        let fun: Box<dyn Fn() -> <V as Val>::Raw + 'd> = Box::new(fun);
        let fun = FunVal::new::<V>(fun);
        self.set.push(fun)
    }
}
