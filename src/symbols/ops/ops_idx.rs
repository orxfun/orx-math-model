use crate::ptr::ValPtr;
use crate::symbols::{IdxDepr, Int, IntFun};
use alloc::boxed::Box;
use core::ops::Add;

// Idx x Idx

impl<'m, 'd> Add<IdxDepr<'m, 'd>> for IdxDepr<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: IdxDepr<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        // SAFETY: both are tied to lifetime of 'm and 'd due to return type
        let [a, b] = unsafe { [ValPtr::new(self.val_ptr()), ValPtr::new(rhs.val_ptr())] };
        let f = move || a.val() + b.val();
        self.m.add_int(IntFun::new(Box::new(f)))
    }
}

// Idx x usize

impl<'m, 'd> Add<usize> for IdxDepr<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: usize) -> Self::Output {
        // SAFETY: both are tied to lifetime of 'm and 'd due to return type
        let a = unsafe { ValPtr::new(self.val_ptr()) };
        let f = move || a.val() + rhs;
        self.m.add_int(IntFun::new(Box::new(f)))
    }
}

impl<'m, 'd> Add<IdxDepr<'m, 'd>> for usize {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: IdxDepr<'m, 'd>) -> Self::Output {
        // SAFETY: both are tied to lifetime of 'm and 'd due to return type
        let b = unsafe { ValPtr::new(rhs.val_ptr()) };
        let f = move || self + b.val();
        rhs.m.add_int(IntFun::new(Box::new(f)))
    }
}
