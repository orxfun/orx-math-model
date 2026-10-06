use core::ops::Index;
use orx_imp_vec::{ImpVec, PinnedVec};

#[derive(derive_new::new, Clone, Copy)]
pub struct Pos(usize);

impl Pos {
    pub fn new_for<D, P: PinnedVec<D>>(vec: &ImpVec<D, P>) -> Self {
        Self(vec.len())
    }
}

impl<D, P: PinnedVec<D>> Index<Pos> for ImpVec<D, P> {
    type Output = D;

    fn index(&self, index: Pos) -> &Self::Output {
        &self[index.0]
    }
}
