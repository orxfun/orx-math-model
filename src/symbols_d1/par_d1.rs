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

// data

#[derive(derive_new::new)]
pub struct ParD1Data<'d> {
    s0: SetRef,
    fun_ptr: *mut ParD1Fun<'d>,
}

impl<'d> ParD1Data<'d> {
    pub fn eval(&self, i0: usize) -> f64 {
        unsafe { &*self.fun_ptr }.eval(i0)
    }

    pub fn update_fun(&mut self, fun: ParD1Fun<'d>) {
        let _ = unsafe { self.fun_ptr.replace(fun) };
    }
}

// symbol

// #[derive(derive_new::new, Clone, Copy)]
// pub struct SetD1<'m, 'd> {
//     pub(crate) m: &'m Model<'d>,
//     pub(crate) el: Elem<'m, SetD1Data<'d>>,
// }
