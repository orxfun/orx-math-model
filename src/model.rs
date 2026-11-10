use crate::bag::{new_bag, Bag};
use crate::index_types::empty_string;
use crate::indices::DIdx;
use crate::pars::{DParD1, IntParD1, NumParD1};
use crate::ptr::ref_eq;
use crate::sets::{CatSet, DSet, OrdSet, Set};
use crate::values::{DVal, Int};
use alloc::boxed::Box;
use alloc::string::String;
use orx_imp_vec::Elem;

pub struct Model<'d> {
    sets: Bag<DSet<'d>>,
    indices: Bag<DIdx<'d>>,
    p1: Bag<DParD1<'d>>,

    // idx slots
    slot_usize: Bag<usize>,
    slot_str: Bag<&'d String>,
    slot_i64: Bag<i64>,

    // values
    val: Bag<DVal<'d>>,

    // value symbols
    sym_int: Bag<Int<'d, 'd>>,
}

impl<'d> Model<'d> {
    pub fn new() -> Self {
        Self {
            sets: new_bag(),
            indices: new_bag(),
            p1: new_bag(),

            // idx slots
            slot_usize: new_bag(),
            slot_str: new_bag(),
            slot_i64: new_bag(),

            // values
            val: new_bag(),

            // value symbols
            sym_int: new_bag(),
        }
    }

    // sets

    pub fn set_cat_int<'m>(&'m self) -> CatSet<'m, 'd> {
        CatSet::new(self, self.sets.push(DSet::new_cat_int()))
    }

    pub fn set_cat_str<'m>(&'m self) -> CatSet<'m, 'd> {
        CatSet::new(self, self.sets.push(DSet::new_cat_str()))
    }

    pub fn set_ord<'m>(&'m self) -> OrdSet<'m, 'd> {
        OrdSet::new(self, self.sets.push(DSet::new_ord()))
    }

    // pars

    pub fn int_par_d1<'m>(&'m self, set: impl Into<Set<'m, 'd>>) -> IntParD1<'m, 'd> {
        IntParD1::new(self, self.p1.push(DParD1::new_int(set.into().el())))
    }

    pub fn num_par_d1<'m>(&'m self, set: impl Into<Set<'m, 'd>>) -> NumParD1<'m, 'd> {
        NumParD1::new(self, self.p1.push(DParD1::new_num(set.into().el())))
    }

    // indices

    pub(crate) fn index<'m>(&'m self, data: DIdx<'d>) -> Elem<'m, DIdx<'d>> {
        self.indices.push(data)
    }

    // idx slots

    pub(crate) fn slot_usize<'m>(&'m self) -> Elem<'m, usize> {
        self.slot_usize.push(Default::default())
    }

    pub(crate) fn slot_str<'m>(&'m self) -> Elem<'m, &'d String> {
        self.slot_str.push(unsafe { empty_string() })
    }

    pub(crate) fn slot_i64<'m>(&'m self) -> Elem<'m, i64> {
        self.slot_i64.push(Default::default())
    }

    // val

    pub(crate) fn val<'m>(&'m self, data: DVal<'d>) -> Elem<'m, DVal<'d>> {
        self.val.push(data)
    }

    pub(crate) fn val2<'m>(&'m self, data: DVal<'d>) -> Elem<'d, DVal<'d>> {
        let ptr = self.val.push(data).as_mut_ptr();
        unsafe { Elem::new(ptr) }
    }

    // value symbols

    pub(crate) fn sym_int<'m>(&'m self, sym: Int<'d, 'd>) -> &'m Int<'d, 'd> {
        let el = self.sym_int.push(sym).as_mut_ptr();
        unsafe { Elem::new(el).get() }
    }
}
