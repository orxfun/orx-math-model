use crate::ptr::{RefPtr, ValPtr};
use crate::symbols::{Idx, Int, IntFun, ParInt};
use alloc::boxed::Box;
use core::ops::Add;

// Int x Int

impl<'m, 'd> Add<Int<'m, 'd>> for Int<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: Int<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        // SAFETY: both are tied to lifetime of 'm and 'd due to return type
        let [a, b] = unsafe { [RefPtr::new(self.fun_ptr()), RefPtr::new(rhs.fun_ptr())] };
        let f = move || a.val().eval() + b.val().eval();
        self.m.int(IntFun::new(Box::new(f)))
    }
}

// Int x Idx

impl<'m, 'd> Add<Idx<'m, 'd>> for Int<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: Idx<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        // SAFETY: both are tied to lifetime of 'm and 'd due to return type
        let (a, b) = unsafe { (RefPtr::new(self.fun_ptr()), ValPtr::new(rhs.val_ptr())) };
        let f = move || a.val().eval() + b.val();
        self.m.int(IntFun::new(Box::new(f)))
    }
}

impl<'m, 'd> Add<Int<'m, 'd>> for Idx<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: Int<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        // SAFETY: both are tied to lifetime of 'm and 'd due to return type
        let (a, b) = unsafe { (ValPtr::new(self.val_ptr()), RefPtr::new(rhs.fun_ptr())) };
        let f = move || a.val() + b.val().eval();
        self.m.int(IntFun::new(Box::new(f)))
    }
}

// Int x ParInt

impl<'m, 'd> Add<Idx<'m, 'd>> for ParInt<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: Idx<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        // SAFETY: both are tied to lifetime of 'm and 'd due to return type

        let (a, b) = unsafe { (ValPtr::new(self.val_ptr()), ValPtr::new(rhs.val_ptr())) };
        let f = move || a.val() + b.val();
        self.m.int(IntFun::new(Box::new(f)))
    }
}

impl<'m, 'd> Add<ParInt<'m, 'd>> for Idx<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: ParInt<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        // SAFETY: both are tied to lifetime of 'm and 'd due to return type
        let (a, b) = unsafe { (ValPtr::new(self.val_ptr()), ValPtr::new(rhs.val_ptr())) };
        let f = move || a.val() + b.val();
        self.m.int(IntFun::new(Box::new(f)))
    }
}

// Int x usize

impl<'m, 'd> Add<usize> for Int<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, b: usize) -> Self::Output {
        // SAFETY: both are tied to lifetime of 'm and 'd due to return type
        let a = unsafe { RefPtr::new(self.fun_ptr()) };
        let f = move || a.val().eval() + b;
        self.m.int(IntFun::new(Box::new(f)))
    }
}

impl<'m, 'd> Add<Int<'m, 'd>> for usize {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: Int<'m, 'd>) -> Self::Output {
        // SAFETY: both are tied to lifetime of 'm and 'd due to return type
        let b = unsafe { RefPtr::new(rhs.fun_ptr()) };
        let f = move || self + b.val().eval();
        rhs.m.int(IntFun::new(Box::new(f)))
    }
}
