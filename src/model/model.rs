use crate::indices::{CatIdx, OrdIdx};
use crate::model::indices::Indices;
use crate::model::numbers::Numbers;
use crate::model::sets::Sets;
use crate::numbers::{Int, Num, ValFun};
use crate::sets::{CatSet, OrdSet};

pub struct Model<'d> {
    sets: Sets<'d>,
    indices: Indices<'d>,
    numbers: Numbers<'d>,
}

impl<'d> Model<'d> {
    pub fn new() -> Self {
        Self {
            sets: Sets::new(),
            indices: Indices::new(),
            numbers: Numbers::new(),
        }
    }

    // sets

    pub fn cat_set_int<'m>(&'m self) -> CatSet<'m, 'd, usize> {
        CatSet::new(self, self.sets.cat_int())
    }

    pub fn cat_set_str<'m>(&'m self) -> CatSet<'m, 'd, &'d str> {
        CatSet::new(self, self.sets.cat_str())
    }

    pub fn ord_set<'m>(&'m self) -> OrdSet<'m, 'd> {
        OrdSet::new(self, self.sets.ord())
    }

    // idx-scalars

    pub(crate) fn idx_cat_int<'m>(&'m self, s: CatSet<'m, 'd, usize>) -> CatIdx<'m, 'd, usize> {
        CatIdx::new(self, self.indices.cat_int(s))
    }

    pub(crate) fn idx_cat_str<'m>(&'m self, s: CatSet<'m, 'd, &'d str>) -> CatIdx<'m, 'd, &'d str> {
        CatIdx::new(self, self.indices.cat_str(s))
    }

    pub(crate) fn idx_ord<'m>(&'m self, s: OrdSet<'m, 'd>) -> OrdIdx<'m, 'd> {
        OrdIdx::new(self, self.indices.ord(s))
    }

    // numbers

    pub(crate) fn numbers_int<'m>(&'m self, fun: ValFun<'d, i64>) -> Int<'m, 'd> {
        Int::new(self, self.numbers.int(fun))
    }

    pub(crate) fn numbers_num<'m>(&'m self, fun: ValFun<'d, f64>) -> Num<'m, 'd> {
        Num::new(self, self.numbers.num(fun))
    }
}
