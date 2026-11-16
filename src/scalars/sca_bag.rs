use crate::scalars::DSca;
use crate::{new_bag, Bag, Ptr, ScaInt, ScaNum};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

pub struct ScaBag<'d> {
    bag: Bag<DSca<'d>>,
    pub minus_one_int: Elem<'d, DSca<'d>>,
    pub one: Elem<'d, DSca<'d>>,
    pub zero_int: Elem<'d, DSca<'d>>,
    pub zero_num: Elem<'d, DSca<'d>>,
}

impl Default for ScaBag<'_> {
    fn default() -> Self {
        let bag = new_bag();

        let [one, minus_one_int, zero_int, zero_num] = [
            DSca::one(),
            DSca::minus_one::<ScaInt>(),
            DSca::zero::<ScaInt>(),
            DSca::zero::<ScaNum>(),
        ]
        .map(|d| unsafe { Elem::new(bag.push(d).as_mut_ptr()) });

        Self {
            bag,
            one,
            minus_one_int,
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
