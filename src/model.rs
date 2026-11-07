use crate::bag::{new_bag, Bag};
use crate::indices::DIdx;
use crate::params::{DParD1, ParD1};
use crate::sets::{CatSet, DSet, OrdSet, Set};
use crate::values::{DVal, DValKinds, Int, Num};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

pub struct Model<'d> {
    sets: Bag<DSet<'d>>,
    indices: Bag<DIdx<'d>>,
    d1_pars_num: Bag<DParD1<'d, f64>>,
    d1_pars_int: Bag<DParD1<'d, i64>>,

    // values
    val_usize: Bag<usize>,
    val_str: Bag<&'d str>,

    // funs
    fun_num: Bag<DVal<'d, f64>>,
    fun_int: Bag<DVal<'d, i64>>,
}

impl<'d> Model<'d> {
    pub fn new() -> Self {
        Self {
            sets: new_bag(),
            indices: new_bag(),
            d1_pars_num: new_bag(),
            d1_pars_int: new_bag(),

            // values
            val_usize: new_bag(),
            val_str: new_bag(),

            // funs
            fun_num: new_bag(),
            fun_int: new_bag(),
        }
    }

    // sets

    pub fn set_cat_int<'m>(&'m self) -> CatSet<'m, 'd, usize> {
        CatSet::new(self, self.sets.push(DSet::empty_cat_int()))
    }

    pub fn set_cat_str<'m>(&'m self) -> CatSet<'m, 'd, &'d str> {
        CatSet::new(self, self.sets.push(DSet::empty_cat_str()))
    }

    pub fn set_ord<'m>(&'m self) -> OrdSet<'m, 'd> {
        OrdSet::new(self, self.sets.push(DSet::empty_ord()))
    }

    // pars

    pub fn par_d1_num<'m>(&'m self, set: impl Into<Set<'m, 'd>>) -> ParD1<'m, 'd, f64> {
        ParD1::new(self, self.d1_pars_num.push(DParD1::empty(set)))
    }

    pub fn par_d1_int<'m>(&'m self, set: impl Into<Set<'m, 'd>>) -> ParD1<'m, 'd, i64> {
        ParD1::new(self, self.d1_pars_int.push(DParD1::empty(set)))
    }

    // indices

    pub(crate) fn index<'m>(&'m self, data: DIdx<'d>) -> Elem<'m, DIdx<'d>> {
        self.indices.push(data)
    }

    // values

    pub(crate) fn val_usize<'m>(&'m self) -> Elem<'m, usize> {
        self.val_usize.push(Default::default())
    }

    pub(crate) fn val_str<'m>(&'m self) -> Elem<'m, &'d str> {
        self.val_str.push(Default::default())
    }

    // funs

    pub(crate) fn fun_num<'m>(&'m self, data: DVal<'d, f64>) -> Num<'m, 'd> {
        Num::new(self, self.fun_num.push(data))
    }

    pub(crate) fn fun_int<'m>(&'m self, data: DVal<'d, i64>) -> Int<'m, 'd> {
        Int::new(self, self.fun_int.push(data))
    }
}
