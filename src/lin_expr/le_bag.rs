use crate::lin_expr::DLinExp;
use crate::{new_bag, Bag};
use orx_imp_vec::Elem;

pub struct LinExpBag<'d> {
    bag: Bag<DLinExp<'d>>,
}

impl Default for LinExpBag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> LinExpBag<'d> {
    pub fn push<'m>(&'m self, data: DLinExp<'d>) -> Elem<'m, DLinExp<'d>> {
        self.bag.push(data)
    }
}
