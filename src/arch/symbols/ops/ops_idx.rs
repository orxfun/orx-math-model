use crate::ptr::ValPtr;
use crate::symbols::{Idx, Int, IntFun};
use alloc::boxed::Box;
use core::ops::Add;

// Idx x Idx

impl<'m, 'd> Add<Idx<'m, 'd>> for Idx<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: Idx<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        // SAFETY: both are tied to lifetime of 'm and 'd due to return type
        let [a, b] = unsafe { [ValPtr::new(self.val_ptr()), ValPtr::new(rhs.val_ptr())] };
        let f = move || a.val() + b.val();
        self.m.int(IntFun::new(Box::new(f)))
    }
}

// Idx x usize

impl<'m, 'd> Add<usize> for Idx<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: usize) -> Self::Output {
        // SAFETY: both are tied to lifetime of 'm and 'd due to return type
        let a = unsafe { ValPtr::new(self.val_ptr()) };
        let f = move || a.val() + rhs;
        self.m.int(IntFun::new(Box::new(f)))
    }
}

impl<'m, 'd> Add<Idx<'m, 'd>> for usize {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: Idx<'m, 'd>) -> Self::Output {
        // SAFETY: both are tied to lifetime of 'm and 'd due to return type
        let b = unsafe { ValPtr::new(rhs.val_ptr()) };
        let f = move || self + b.val();
        rhs.m.int(IntFun::new(Box::new(f)))
    }
}
