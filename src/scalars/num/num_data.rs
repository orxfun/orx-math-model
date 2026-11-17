use crate::scalars::num::{FInt, FNum};
use crate::ValTyp;
use alloc::boxed::Box;
use core::mem::{transmute, ManuallyDrop};

#[derive(derive_new::new)]
pub struct DNum<'d> {
    typ: ValTyp,
    fun: FNum<'d>,
}

impl<'d> DNum<'d> {
    pub fn new_int(fun: impl Fn() -> i64 + 'd) -> Self {
        todo!()
    }

    #[inline(always)]
    fn eval(&self) -> f64 {
        (self.fun)()
    }
}
