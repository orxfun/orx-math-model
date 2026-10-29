use crate::ptr::RefPtr;
use crate::symbols::{Int, Num, NumFun};
use alloc::boxed::Box;
use core::ops::Add;

// Num x Num

impl<'m, 'd> Add<Num<'m, 'd>> for Num<'m, 'd> {
    type Output = Num<'m, 'd>;

    fn add(self, rhs: Num<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        // SAFETY: both are tied to lifetime of 'm and 'd due to return type
        let [a, b] = unsafe { [RefPtr::new(self.fun_ptr()), RefPtr::new(rhs.fun_ptr())] };
        let f = move || a.val().eval() + b.val().eval();
        self.m.num(NumFun::new(Box::new(f)))
    }
}

// Num x Int

impl<'m, 'd> Add<Int<'m, 'd>> for Num<'m, 'd> {
    type Output = Num<'m, 'd>;

    fn add(self, rhs: Int<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        // SAFETY: both are tied to lifetime of 'm and 'd due to return type
        let (a, b) = unsafe { (RefPtr::new(self.fun_ptr()), RefPtr::new(rhs.fun_ptr())) };
        let f = move || a.val().eval() + b.val().eval() as f64;
        self.m.num(NumFun::new(Box::new(f)))
    }
}

impl<'m, 'd> Add<Num<'m, 'd>> for Int<'m, 'd> {
    type Output = Num<'m, 'd>;

    fn add(self, rhs: Num<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        // SAFETY: both are tied to lifetime of 'm and 'd due to return type
        let (a, b) = unsafe { (RefPtr::new(self.fun_ptr()), RefPtr::new(rhs.fun_ptr())) };
        let f = move || a.val().eval() as f64 + b.val().eval();
        self.m.num(NumFun::new(Box::new(f)))
    }
}

// Num x f64

impl<'m, 'd> Add<f64> for Num<'m, 'd> {
    type Output = Num<'m, 'd>;

    fn add(self, b: f64) -> Self::Output {
        // SAFETY: both are tied to lifetime of 'm and 'd due to return type
        let a = unsafe { RefPtr::new(self.fun_ptr()) };
        let f = move || a.val().eval() + b;
        self.m.num(NumFun::new(Box::new(f)))
    }
}

impl<'m, 'd> Add<Num<'m, 'd>> for f64 {
    type Output = Num<'m, 'd>;

    fn add(self, rhs: Num<'m, 'd>) -> Self::Output {
        // SAFETY: both are tied to lifetime of 'm and 'd due to return type
        let b = unsafe { RefPtr::new(rhs.fun_ptr()) };
        let f = move || self + b.val().eval();
        rhs.m.num(NumFun::new(Box::new(f)))
    }
}

// Num x usize

impl<'m, 'd> Add<usize> for Num<'m, 'd> {
    type Output = Num<'m, 'd>;

    fn add(self, b: usize) -> Self::Output {
        // SAFETY: both are tied to lifetime of 'm and 'd due to return type
        let a = unsafe { RefPtr::new(self.fun_ptr()) };
        let f = move || a.val().eval() + b as f64;
        self.m.num(NumFun::new(Box::new(f)))
    }
}

impl<'m, 'd> Add<Num<'m, 'd>> for usize {
    type Output = Num<'m, 'd>;

    fn add(self, rhs: Num<'m, 'd>) -> Self::Output {
        // SAFETY: both are tied to lifetime of 'm and 'd due to return type
        let b = unsafe { RefPtr::new(rhs.fun_ptr()) };
        let f = move || self as f64 + b.val().eval();
        rhs.m.num(NumFun::new(Box::new(f)))
    }
}
