use super::bag::{new_bag, Bag};
use crate::indices::{CatIdxData, OrdIdxData};
use crate::sets::{CatSet, OrdSet};
use orx_imp_vec::Elem;

pub struct Indices<'d> {
    cat_int: Bag<CatIdxData<'d, usize>>,
    cat_str: Bag<CatIdxData<'d, &'d str>>,
    ord: Bag<OrdIdxData<'d>>,
}

impl<'d> Indices<'d> {
    pub fn new() -> Self {
        Self {
            cat_int: new_bag(),
            cat_str: new_bag(),
            ord: new_bag(),
        }
    }

    pub fn cat_int<'m>(&'m self, s: CatSet<'m, 'd, usize>) -> Elem<'m, CatIdxData<'d, usize>> {
        self.cat_int.push(CatIdxData::new(s))
    }

    pub fn cat_str<'m>(&'m self, s: CatSet<'m, 'd, &'d str>) -> Elem<'m, CatIdxData<'d, &'d str>> {
        self.cat_str.push(CatIdxData::new(s))
    }

    pub fn ord<'m>(&'m self, s: OrdSet<'m, 'd>) -> Elem<'m, OrdIdxData<'d>> {
        self.ord.push(OrdIdxData::new(s))
    }
}
