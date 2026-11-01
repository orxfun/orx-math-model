use super::bag::{new_bag, Bag};
use crate::parameters::ParD1Fun;
use crate::sets::{CategoricalSetInt, CategoricalSetStr, OrdinalSet};
use orx_imp_vec::Elem;

pub struct ParsD1<'d> {
    cat_int_int: Bag<ParD1Fun<'d, CategoricalSetInt, i64>>,
    cat_int_num: Bag<ParD1Fun<'d, CategoricalSetInt, f64>>,
    cat_str_int: Bag<ParD1Fun<'d, CategoricalSetStr, i64>>,
    cat_str_num: Bag<ParD1Fun<'d, CategoricalSetStr, f64>>,
    ord_int: Bag<ParD1Fun<'d, OrdinalSet, i64>>,
    ord_num: Bag<ParD1Fun<'d, OrdinalSet, f64>>,
}

impl<'d> ParsD1<'d> {
    pub fn new() -> Self {
        Self {
            cat_int_int: new_bag(),
            cat_int_num: new_bag(),
            cat_str_int: new_bag(),
            cat_str_num: new_bag(),
            ord_int: new_bag(),
            ord_num: new_bag(),
        }
    }
}
