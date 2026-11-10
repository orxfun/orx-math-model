use crate::bag::{new_bag, Bag};
use crate::indices::DIdx;
use crate::pars::DParD1;
use crate::ptr::ref_eq;
use crate::sets::{CatSet2, DSet, OrdSet};
use crate::values::DVal;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

pub struct Model<'d> {
    sets: Bag<DSet<'d>>,
    indices: Bag<DIdx<'d>>,
    p1: Bag<DParD1<'d>>,
    // d1_pars_num: Bag<DParD1<'d, f64>>,
    // d1_pars_int: Bag<DParD1<'d, i64>>,

    // idx slots
    slot_usize: Bag<usize>,
    slot_str: Bag<&'d str>,
    slot_i64: Bag<i64>,

    // values
    val: Bag<DVal<'d>>,
}

impl<'d> Model<'d> {
    pub fn new() -> Self {
        Self {
            sets: new_bag(),
            indices: new_bag(),
            p1: new_bag(),
            // d1_pars_num: new_bag(),
            // d1_pars_int: new_bag(),

            // idx slots
            slot_usize: new_bag(),
            slot_str: new_bag(),
            slot_i64: new_bag(),

            // values
            val: new_bag(),
        }
    }

    // sets

    pub fn set_cat_int<'m>(&'m self) -> CatSet2<'m, 'd> {
        CatSet2::new(self, self.sets.push(DSet::new_cat_int()))
    }

    pub fn set_cat_str<'m>(&'m self) -> CatSet2<'m, 'd> {
        CatSet2::new(self, self.sets.push(DSet::new_cat_str()))
    }

    pub fn set_ord<'m>(&'m self) -> OrdSet<'m, 'd> {
        OrdSet::new(self, self.sets.push(DSet::new_ord()))
    }

    // pars

    // pub fn int_par_d1<'m>(&'m self, )

    // pub fn par_d1_num<'m>(&'m self, set: impl Into<Set<'m, 'd>>) -> ParD1<'m, 'd, f64> {
    //     ParD1::new(self, self.d1_pars_num.push(DParD1::empty(set)))
    // }

    // pub fn par_d1_int<'m>(&'m self, set: impl Into<Set<'m, 'd>>) -> ParD1<'m, 'd, i64> {
    //     ParD1::new(self, self.d1_pars_int.push(DParD1::empty(set)))
    // }

    // indices

    pub(crate) fn index<'m>(&'m self, data: DIdx<'d>) -> Elem<'m, DIdx<'d>> {
        self.indices.push(data)
    }

    // idx slots

    pub(crate) fn slot_usize<'m>(&'m self) -> Elem<'m, usize> {
        self.slot_usize.push(Default::default())
    }

    pub(crate) fn slot_str<'m>(&'m self) -> Elem<'m, &'d str> {
        self.slot_str.push(Default::default())
    }

    pub(crate) fn slot_i64<'m>(&'m self) -> Elem<'m, i64> {
        self.slot_i64.push(Default::default())
    }

    // val

    pub(crate) fn val<'m>(&'m self, data: DVal<'d>) -> Elem<'m, DVal<'d>> {
        self.val.push(data)
    }
}
