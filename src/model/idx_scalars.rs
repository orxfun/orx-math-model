use super::bag::{new_bag, Bag};
use crate::{idx::IdxData, sets::Set};
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

    pub fn int<'m>(&'m self, s: Set<'m, 'd, usize>) -> Elem<'m, IdxData<'d, usize>> {
        self.int.push(IdxData::new(s))
    }

    pub fn str<'m>(&'m self, s: Set<'m, 'd, &'d str>) -> Elem<'m, IdxData<'d, &'d str>> {
        self.str.push(IdxData::new(s))
    }
}
