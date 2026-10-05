use crate::{model::Model, pars::IntPar0Data};
use orx_imp_vec::{ImpVec, PinnedVec};

#[derive(derive_new::new)]
pub struct Symbol<'a, D> {
    pub m: &'a Model,
    pub d: &'a D,
}

impl<'a, D> Clone for Symbol<'a, D> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, D> Copy for Symbol<'a, D> {}

impl<'a, D> Symbol<'a, D> {
    pub fn destruct(self) -> (&'a Model, &'a D) {
        (self.m, self.d)
    }

    fn pos_in(self, list: &ImpVec<D>) -> Option<usize> {
        list.index_of(self.d)
    }
}

impl<'a> Symbol<'a, IntPar0Data> {
    pub fn pos(self) -> usize {
        self.m.int_par0.index_of(self.d).expect("TODO")
    }
}
