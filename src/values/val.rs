use crate::{ptr::RefPtr, values::Value, Model};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// data

#[derive(derive_new::new)]
pub struct DVal<'d, T: Value> {
    fun: Box<dyn Fn() -> T + 'd>,
}

impl<'d, T: Value> DVal<'d, T> {
    pub fn eval(&self) -> T {
        (self.fun)()
    }
}

// sym

#[derive(derive_new::new, Clone, Copy)]
#[new(visibility = "pub(crate)")]
pub struct Val<'m, 'd, T: Value> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, DVal<'d, T>>,
}

impl<'m, 'd, T: Value> Val<'m, 'd, T> {
    pub(crate) fn fun_ptr(self) -> RefPtr<Box<dyn Fn() -> T + 'd>> {
        todo!()
    }
}
