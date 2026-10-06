use crate::Model;
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
    fn pos_in(self, list: &ImpVec<D>) -> Option<usize> {
        list.index_of(self.d)
    }
}
