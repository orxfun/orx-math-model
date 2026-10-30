use super::bag::{new_bag, Bag};
use crate::idx_scalars::{IdxData, IndicesIdxData};
use crate::sets::{Indices, Set};
use orx_imp_vec::Elem;

pub struct IdxScalars<'d> {
    int: Bag<IdxData<'d, usize>>,
    str: Bag<IdxData<'d, &'d str>>,
    indices: Bag<IndicesIdxData<'d>>,
}

impl<'d> IdxScalars<'d> {
    pub fn new() -> Self {
        Self {
            int: new_bag(),
            str: new_bag(),
            indices: new_bag(),
        }
    }

    pub fn int<'m>(&'m self, s: Set<'m, 'd, usize>) -> Elem<'m, IdxData<'d, usize>> {
        self.int.push(IdxData::new(s))
    }

    pub fn str<'m>(&'m self, s: Set<'m, 'd, &'d str>) -> Elem<'m, IdxData<'d, &'d str>> {
        self.str.push(IdxData::new(s))
    }

    pub fn indices<'m>(&'m self, s: Indices<'m, 'd>) -> Elem<'m, IndicesIdxData<'d>> {
        self.indices.push(IndicesIdxData::new(s))
    }
}
