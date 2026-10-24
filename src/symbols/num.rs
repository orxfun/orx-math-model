use crate::ptr::{RefPtr, ValPtr};
use crate::symbols::{Int, Par};
use crate::Model;
use alloc::boxed::Box;
use core::ops::Add;
use orx_imp_vec::Elem;

#[derive(derive_new::new)]
pub struct NumFun<'d>(Box<dyn Fn() -> f64 + 'd>);

impl NumFun<'_> {
    pub fn eval(&self) -> f64 {
        self.0()
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct Num<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, NumFun<'d>>,
}

impl<'d> Num<'_, 'd> {
    pub(crate) fn fun_ptr(&self) -> *const NumFun<'d> {
        unsafe { self.el.get() }
    }
}

// from

impl<'m, 'd> From<Par<'m, 'd>> for Num<'m, 'd> {
    fn from(value: Par<'m, 'd>) -> Self {
        let p = unsafe { ValPtr::new(value.val_ptr()) };
        let fun = NumFun(Box::new(move || p.val()));
        value.m.add_num(fun)
    }
}
