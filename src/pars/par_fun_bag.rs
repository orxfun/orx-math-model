use crate::pars::ParFun;
use crate::{new_bag, Bag};
use orx_imp_vec::Elem;

pub struct ParFunBag<'d> {
    set: Bag<ParFun<'d>>,
}

impl Default for ParFunBag<'_> {
    fn default() -> Self {
        Self { set: new_bag() }
    }
}

impl<'d> ParFunBag<'d> {
    pub fn push<'m>(&'m self, fun: ParFun<'d>) -> Elem<'m, ParFun<'d>> {
        self.set.push(fun)
    }
}
