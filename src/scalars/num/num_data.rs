use crate::scalars::num::{meta_val::ValTyp, num_fun::FNum};
use alloc::boxed::Box;
use core::mem::{transmute, ManuallyDrop};

#[derive(derive_new::new)]
pub struct DNum<'d> {
    typ: ValTyp,
    fun: FNum<'d>,
}

impl<'d> DNum<'d> {
    #[inline(always)]
    fn eval(&self) -> f64 {
        (self.fun)()
    }
}
