use crate::symbols::{Int, IntFun, Num, NumFun, PseudoInt, PtrIntFun, RefSetOfIdx};
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// fun

#[derive(derive_new::new)]
pub struct ParCoreD1Fun<'d, T: Default>(Box<dyn Fn(usize) -> T + 'd>);

impl<T: Default> ParCoreD1Fun<'_, T> {
    pub fn empty() -> Self {
        Self(Box::new(|_| Default::default()))
    }

    pub fn eval(&self, i0: usize) -> T {
        self.0(i0)
    }
}

// data

#[derive(derive_new::new)]
pub struct ParCoreD1Data<'d, T: Default> {
    s0: RefSetOfIdx,
    fun: ParCoreD1Fun<'d, T>,
}

impl<'d, T: Default> ParCoreD1Data<'d, T> {
    pub fn empty(s0: RefSetOfIdx) -> Self {
        Self::new(s0, ParCoreD1Fun::empty())
    }

    pub fn eval(&self, i0: usize) -> T {
        self.fun.eval(i0)
    }

    pub fn update_fun(&mut self, fun: ParCoreD1Fun<'d, T>) {
        self.fun = fun;
    }

    pub fn ptr_fun(p: *const ParCoreD1Data<'d, T>) -> *const ParCoreD1Fun<'d, T> {
        unsafe { core::ptr::addr_of!((*p).fun) }
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct ParCoreD1<'m, 'd, T: Default> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, ParCoreD1Data<'d, T>>,
}

// number

pub type ParD1Fun<'d> = ParCoreD1Fun<'d, f64>;

pub type ParD1Data<'d> = ParCoreD1Data<'d, f64>;

pub type ParD1<'m, 'd> = ParCoreD1<'m, 'd, f64>;

impl<'m, 'd> ParD1<'m, 'd> {
    pub(crate) fn at(self, i: impl Into<PseudoInt<'m, 'd>>) -> Num<'m, 'd> {
        struct PtrParD1Fun<'d, T: Default>(*const ParCoreD1Fun<'d, T>);

        impl<'d, T: Default> PtrParD1Fun<'d, T> {
            /// # SAFETY
            ///
            /// * Created value must not outlive `'m` or `'d`.
            unsafe fn new<'m>(set: ParCoreD1<'m, 'd, T>) -> Self {
                Self(core::ptr::addr_of!((*set.el.as_ptr()).fun))
            }

            fn val(&self) -> &ParCoreD1Fun<'d, T> {
                unsafe { &*self.0 }
            }
        }

        // SAFETY: fun_d1 is tied to lifetime of 'm and 'd due to return type Num<'m, 'd>.
        let fun_d1 = unsafe { PtrParD1Fun::new(self) };

        let f = match i.into() {
            PseudoInt::Int(i) => {
                assert!(core::ptr::eq(self.m, i.m), "TODO: error msg");
                // SAFETY: p is tied to lifetime of 'm and 'd due to return type Num<'m, 'd>.
                let p = unsafe { PtrIntFun::new(i) };
                NumFun::new(Box::new(move || fun_d1.val().0(p.fun().eval())))
            }
            PseudoInt::Raw(i0) => NumFun::new(Box::new(move || fun_d1.val().0(i0))),
        };
        self.m.add_num(f)
    }
}

// int

pub type ParIntD1Fun<'d> = ParCoreD1Fun<'d, usize>;

pub type ParIntD1Data<'d> = ParCoreD1Data<'d, usize>;

pub type ParIntD1<'m, 'd> = ParCoreD1<'m, 'd, usize>;

impl<'m, 'd> ParIntD1<'m, 'd> {
    pub(crate) fn at(self, i: impl Into<PseudoInt<'m, 'd>>) -> Int<'m, 'd> {
        struct PtrParD1Fun<'d, T: Default>(*const ParCoreD1Fun<'d, T>);

        impl<'d, T: Default> PtrParD1Fun<'d, T> {
            /// # SAFETY
            ///
            /// * Created value must not outlive `'m` or `'d`.
            unsafe fn new<'m>(set: ParCoreD1<'m, 'd, T>) -> Self {
                Self(core::ptr::addr_of!((*set.el.as_ptr()).fun))
            }

            fn val(&self) -> &ParCoreD1Fun<'d, T> {
                unsafe { &*self.0 }
            }
        }

        // SAFETY: fun_d1 is tied to lifetime of 'm and 'd due to return type Num<'m, 'd>.
        let fun_d1 = unsafe { PtrParD1Fun::new(self) };

        let f = match i.into() {
            PseudoInt::Int(i) => {
                assert!(core::ptr::eq(self.m, i.m), "TODO: error msg");
                // SAFETY: p is tied to lifetime of 'm and 'd due to return type Num<'m, 'd>.
                let p = unsafe { PtrIntFun::new(i) };
                IntFun::new(Box::new(move || fun_d1.val().0(p.fun().eval())))
            }
            PseudoInt::Raw(i0) => IntFun::new(Box::new(move || fun_d1.val().0(i0))),
        };
        self.m.add_int(f)
    }
}
