use crate::scalars::DSca;
use crate::{new_bag, Bag};
use orx_imp_vec::Elem;

pub struct ScaBag<'d> {
    bag: Bag<DSca<'d>>,
}

impl Default for ScaBag<'_> {
    fn default() -> Self {
        Self { bag: new_bag() }
    }
}

impl<'d> ScaBag<'d> {
    pub fn push<'m>(&'m self, data: DSca<'d>) -> Elem<'m, DSca<'d>> {
        self.bag.push(data)
    }
}
