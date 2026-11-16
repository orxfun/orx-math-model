use crate::scalars::DSca;
use crate::{new_bag, Bag, ScaInt, ScaNum};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

pub struct ScaBag<'d> {
    bag: Bag<DSca<'d>>,
}

impl Default for ScaBag<'_> {
    fn default() -> Self {
        let bag = new_bag();

        let one = DSca::one();
        let zero_int = DSca::zero::<ScaInt>();
        let zero_num = DSca::zero::<ScaNum>();

        let one = DSca::new_val::<ScaNum>(Box::new(|| 1.0));

        Self { bag }
    }
}

impl<'d> ScaBag<'d> {
    pub fn push<'m>(&'m self, data: DSca<'d>) -> Elem<'m, DSca<'d>> {
        self.bag.push(data)
    }
}
