use crate::scalars::DSca;
use crate::{new_bag, Bag, Ptr, ScaInt, ScaNum};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

pub struct ScaBag<'d> {
    bag: Bag<DSca<'d>>,
    one: Ptr<'d, DSca<'d>>,
    zero_int: Ptr<'d, DSca<'d>>,
    zero_num: Ptr<'d, DSca<'d>>,
}

impl Default for ScaBag<'_> {
    fn default() -> Self {
        let bag = new_bag();

        let [one, zero_int, zero_num] =
            [DSca::one(), DSca::zero::<ScaInt>(), DSca::zero::<ScaNum>()]
                .map(|d| Ptr::new_from_el(bag.push(d)));

        Self {
            bag,
            one,
            zero_int,
            zero_num,
        }
    }
}

impl<'d> ScaBag<'d> {
    pub fn push<'m>(&'m self, data: DSca<'d>) -> Elem<'m, DSca<'d>> {
        self.bag.push(data)
    }
}
