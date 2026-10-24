use crate::symbols::{Idx, IdxData, PtrIdxValue};
use crate::Model;
use alloc::boxed::Box;
use core::ops::Add;
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

impl<'m, 'd> From<Idx<'m, 'd>> for Int<'m, 'd> {
    fn from(value: Idx<'m, 'd>) -> Self {
        let p = IdxData::ptr_value(value.el.as_ptr());
        let fun = IntFun(Box::new(move || unsafe { *p }));
        value.m.add_int(fun)
    }
}

// ref

pub struct PtrIntFun<'d>(*const IntFun<'d>);

impl<'d> PtrIntFun<'d> {
    /// # SAFETY
    ///
    /// * Created value must not outlive `'m` or `'d`.
    pub unsafe fn new<'m>(int: Int<'m, 'd>) -> Self {
        Self(unsafe { int.el.get() })
    }

    pub fn fun(&self) -> &IntFun<'d> {
        unsafe { &*self.0 }
    }
}

// ops - Int

impl<'m, 'd> Add<Int<'m, 'd>> for Int<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: Int<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Sca<'m, 'd>.
        let [a, b] = unsafe { [PtrIntFun::new(self), PtrIntFun::new(rhs)] };
        let f = move || a.fun().eval() + b.fun().eval();
        self.m.add_int(IntFun(Box::new(f)))
    }
}

// ops - Idx

impl<'m, 'd> Add<Idx<'m, 'd>> for Int<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: Idx<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Sca<'m, 'd>.
        let (a, b) = unsafe { (PtrIntFun::new(self), PtrIdxValue::new(rhs)) };
        let f = move || a.fun().eval() + b.value();
        self.m.add_int(IntFun(Box::new(f)))
    }
}

impl<'m, 'd> Add<Int<'m, 'd>> for Idx<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: Int<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Sca<'m, 'd>.
        let (a, b) = unsafe { (PtrIdxValue::new(self), PtrIntFun::new(rhs)) };
        let f = move || a.value() + b.fun().eval();
        self.m.add_int(IntFun(Box::new(f)))
    }
}

// ops - usize

impl<'m, 'd> Add<usize> for Int<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, b: usize) -> Self::Output {
        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Sca<'m, 'd>.
        let a = unsafe { PtrIntFun::new(self) };
        let f = move || a.fun().eval() + b;
        self.m.add_int(IntFun(Box::new(f)))
    }
}

impl<'m, 'd> Add<Int<'m, 'd>> for usize {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: Int<'m, 'd>) -> Self::Output {
        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Sca<'m, 'd>.
        let b = unsafe { PtrIntFun::new(rhs) };
        let f = move || self + b.fun().eval();
        rhs.m.add_int(IntFun(Box::new(f)))
    }
}
