use super::bag::{new_bag, Bag};
use crate::parameters::ParD1Fun;
use crate::sets::{CatSetFun, OrdSetFun};
use orx_imp_vec::Elem;

pub struct ParsD1<'d> {
    // cat_int_int: Bag<ParD1Fun<'d, Cat>>
    cat_int: Bag<CatSetFun<'d, usize>>,
    cat_str: Bag<CatSetFun<'d, &'d str>>,
    ord: Bag<OrdSetFun<'d>>,
}
