use super::bag::{new_bag, Bag};
use crate::idx::IdxData;
use orx_imp_vec::Elem;

pub struct IdxScalars<'d> {
    int: Bag<IdxData<'d, usize>>,
    str: Bag<IdxData<'d, &'d str>>,
}

impl<'d> IdxScalars<'d> {
    pub fn new() -> Self {
        Self {
            int: new_bag(),
            str: new_bag(),
        }
    }
}
