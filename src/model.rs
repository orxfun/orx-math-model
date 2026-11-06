use crate::bag::{new_bag, Bag};
use crate::indices::DIdx;
use crate::params::{DParD1, ParD1};
use crate::sets::{CatSet, DSet, OrdSet, Set};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

pub struct Model<'d> {
    sets: Bag<DSet<'d>>,
    indices: Bag<DIdx<'d>>,
    d1_pars: Bag<DParD1<'d>>,

    // values
    val_usize: Bag<usize>,
    val_str: Bag<&'d str>,

    // funs
    fun_f64: Bag<Box<dyn Fn() -> f64 + 'd>>,
    fun_i64: Bag<Box<dyn Fn() -> i64 + 'd>>,
}

impl<'d> Model<'d> {
    pub fn new() -> Self {
        Self {
            sets: new_bag(),
            indices: new_bag(),
            d1_pars: new_bag(),

            // values
            val_usize: new_bag(),
            val_str: new_bag(),

            // funs
            fun_f64: new_bag(),
            fun_i64: new_bag(),
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

    pub fn par_d1_num<'m>(&'m self, set: impl Into<Set<'m, 'd>>) -> ParD1<'m, 'd> {
        ParD1::new(self, self.d1_pars.push(DParD1::empty_num(set)))
    }

    pub fn par_d1_int<'m>(&'m self, set: impl Into<Set<'m, 'd>>) -> ParD1<'m, 'd> {
        ParD1::new(self, self.d1_pars.push(DParD1::empty_int(set)))
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

    pub(crate) fn fun_f64<'m>(
        &'m self,
        fun: Box<dyn Fn() -> f64 + 'd>,
    ) -> Elem<'m, Box<dyn Fn() -> f64 + 'd>> {
        self.fun_f64.push(fun)
    }

    pub(crate) fn fun_i64<'m>(
        &'m self,
        fun: Box<dyn Fn() -> i64 + 'd>,
    ) -> Elem<'m, Box<dyn Fn() -> i64 + 'd>> {
        self.fun_i64.push(fun)
    }
}
