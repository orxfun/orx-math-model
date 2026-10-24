use crate::ptr::ValPtr;
use crate::symbols::{Idx, IdxData, ParInt};
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// fun

#[derive(derive_new::new)]
pub struct IntFun<'d>(Box<dyn Fn() -> usize + 'd>);

impl IntFun<'_> {
    pub fn eval(&self) -> usize {
        self.0()
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct Int<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, IntFun<'d>>,
}

impl<'d> Int<'_, 'd> {
    pub fn fun_ptr(&self) -> *const IntFun<'d> {
        unsafe { self.el.get() }
    }
}

// from

impl<'m, 'd> From<Idx<'m, 'd>> for Int<'m, 'd> {
    fn from(value: Idx<'m, 'd>) -> Self {
        let p = IdxData::ptr_value(value.el.as_ptr());
        let fun = IntFun(Box::new(move || unsafe { *p }));
        value.m.add_int(fun)
    }
}

impl<'m, 'd> From<ParInt<'m, 'd>> for Int<'m, 'd> {
    fn from(value: ParInt<'m, 'd>) -> Self {
        let p = unsafe { ValPtr::new(value.val_ptr()) };
        let fun = IntFun(Box::new(move || p.val()));
        value.m.add_int(fun)
    }
}
