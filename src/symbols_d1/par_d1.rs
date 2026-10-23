use crate::symbols::{Idx, Int, Num, NumFun, PtrIdxValue, PtrIntFun, SetRef};
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
    fun: ParD1Fun<'d>,
}

impl<'d> ParD1Data<'d> {
    pub fn eval(&self, i0: usize) -> f64 {
        self.fun.eval(i0)
    }

    pub fn update_fun(&mut self, fun: ParD1Fun<'d>) {
        self.fun = fun;
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct ParD1<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, ParD1Data<'d>>,
}

impl<'m, 'd> ParD1<'m, 'd> {
    pub(crate) fn sca_at2(self, i: Int<'m, 'd>) -> Num<'m, 'd> {
        struct PtrParD1Fun<'d>(*const ParD1Fun<'d>);

        impl<'d> PtrParD1Fun<'d> {
            /// # SAFETY
            ///
            /// * Created value must not outlive `'m` or `'d`.
            unsafe fn new<'m>(set: ParD1<'m, 'd>) -> Self {
                Self(core::ptr::addr_of!((*set.el.as_ptr()).fun))
            }

            fn value(&self) -> &ParD1Fun<'d> {
                unsafe { &*self.0 }
            }
        }

        assert!(core::ptr::eq(self.m, i.m), "TODO: error msg");

        let s1 = unsafe { self.el.get() };
        let idx = unsafe { i.el.get() };

        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Set<'m, 'd>.
        let (p, fun_d1) = unsafe { (PtrIntFun::new(i), PtrParD1Fun::new(self)) };
        let f = move || {
            let i0 = p.fun();
            let fun = fun_d1.value();
            fun.0(i0.eval())
        };

        self.m.add_num(NumFun::new(Box::new(f)))
    }

    pub(crate) fn sca_at(self, i: Idx<'m, 'd>) -> Num<'m, 'd> {
        struct PtrParD1Fun<'d>(*const ParD1Fun<'d>);

        impl<'d> PtrParD1Fun<'d> {
            /// # SAFETY
            ///
            /// * Created value must not outlive `'m` or `'d`.
            unsafe fn new<'m>(set: ParD1<'m, 'd>) -> Self {
                Self(core::ptr::addr_of!((*set.el.as_ptr()).fun))
            }

            fn value(&self) -> &ParD1Fun<'d> {
                unsafe { &*self.0 }
            }
        }

        assert!(core::ptr::eq(self.m, i.m), "TODO: error msg");

        let s1 = unsafe { self.el.get() };
        let idx = unsafe { i.el.get() };
        assert_eq!(s1.s0, idx.s0(), "TODO: err-msg");

        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Set<'m, 'd>.
        let (p, fun_d1) = unsafe { (PtrIdxValue::new(i), PtrParD1Fun::new(self)) };
        let f = move || {
            let i0 = p.value();
            let fun = fun_d1.value();
            fun.0(i0)
        };

        self.m.add_num(NumFun::new(Box::new(f)))
    }
}
