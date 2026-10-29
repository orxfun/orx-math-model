use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// fun

#[derive(derive_new::new)]
pub struct ValFun<'d, T>(Box<dyn Fn() -> T + 'd>);

impl<T> ValFun<'_, T> {
    pub fn eval(&self) -> T {
        self.0()
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct Val<'m, 'd, T> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, ValFun<'d, T>>,
}

impl<'d, T> Val<'_, 'd, T> {
    pub(crate) fn fun_ptr(&self) -> *const ValFun<'d, T> {
        unsafe { self.el.get() }
    }
}
