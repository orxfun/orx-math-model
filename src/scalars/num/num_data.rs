use crate::scalars::num::{FInt, FNum};
use crate::{AsF64, AsI64, ValTyp};
use alloc::boxed::Box;
use core::mem::{transmute, ManuallyDrop};

#[derive(derive_new::new)]
pub struct DNum<'d> {
    typ: ValTyp,
    fun: FNum<'d>,
}

impl<'d> DNum<'d> {
    pub fn new_num<V: AsF64>(fun: impl Fn() -> V + 'd) -> Self {
        let fun = Box::new(move || fun().as_f64());
        let typ = ValTyp::Num;
        Self { typ, fun }
    }

    pub fn new_int<V: AsI64>(fun: impl Fn() -> V + 'd) -> Self {
        let fun = Box::new(move || fun().as_f64());
        let typ = ValTyp::Int;
        Self { typ, fun }
    }

    #[inline(always)]
    fn eval(&self) -> f64 {
        (self.fun)()
    }

    fn eval_int(&self) -> i64 {
        todo!()
    }
}
