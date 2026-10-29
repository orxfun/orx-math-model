use crate::ptr::RefPtr;
use crate::symbols::{Int, IntFun, ParInt};
use alloc::boxed::Box;
use core::ops::Add;

// ParInt x ParInt

impl<'m, 'd> Add<ParInt<'m, 'd>> for ParInt<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: ParInt<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        // SAFETY: both are tied to lifetime of 'm and 'd due to return type
        let [a, b] = unsafe { [RefPtr::new(self.val_ptr()), RefPtr::new(rhs.val_ptr())] };
        let f = move || a.val() + b.val();
        self.m.int(IntFun::new(Box::new(f)))
    }
}

// ParInt x usize

impl<'m, 'd> Add<usize> for ParInt<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, b: usize) -> Self::Output {
        // SAFETY: both are tied to lifetime of 'm and 'd due to return type
        let a = unsafe { RefPtr::new(self.val_ptr()) };
        let f = move || a.val() + b;
        self.m.int(IntFun::new(Box::new(f)))
    }
}

impl<'m, 'd> Add<ParInt<'m, 'd>> for usize {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: ParInt<'m, 'd>) -> Self::Output {
        // SAFETY: both are tied to lifetime of 'm and 'd due to return type
        let b = unsafe { RefPtr::new(rhs.val_ptr()) };
        let f = move || self + b.val();
        rhs.m.int(IntFun::new(Box::new(f)))
    }
}
