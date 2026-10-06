use crate::Model;
use alloc::boxed::Box;

// data

#[derive(derive_new::new)]
pub struct FunIntData {
    fun: Box<dyn Fn(&Model) -> i64>,
}

impl FunIntData {
    pub fn value(&self, m: &Model) -> i64 {
        (self.fun)(m)
    }
}
