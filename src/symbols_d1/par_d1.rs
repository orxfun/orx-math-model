use crate::symbols::{Idx, RefIdxValue, Set, SetFun, SetRef};
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// fun

#[derive(derive_new::new)]
pub struct ParD1Fun<'d>(Box<dyn Fn(usize) -> f64 + 'd>);

impl ParD1Fun<'_> {
    pub fn empty() -> Self {
        Self(Box::new(|_| 0.0))
    }

    pub fn eval(&self, i0: usize) -> f64 {
        self.0(i0)
    }
}
