use crate::lin_expr::DLinExpr;
use crate::{new_bag, Bag};
use orx_imp_vec::Elem;

pub struct LinExpBag<'d> {
    bag: Bag<DLinExpr<'d>>,
}

impl Default for LinExpBag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> LinExpBag<'d> {
    pub fn push<'m>(&'m self, data: DLinExpr<'d>) -> Elem<'m, DLinExpr<'d>> {
        self.bag.push(data)
    }
}
