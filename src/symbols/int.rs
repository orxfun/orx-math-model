use crate::ptr::{RefPtr, ValPtr};
use crate::symbols::{Idx, IdxData, ParInt};
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

// ops - Int

impl<'m, 'd> Add<Int<'m, 'd>> for Int<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: Int<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Sca<'m, 'd>.
        let [a, b] = unsafe { [RefPtr::new(self.fun_ptr()), RefPtr::new(rhs.fun_ptr())] };
        let f = move || a.val().eval() + b.val().eval();
        self.m.add_int(IntFun(Box::new(f)))
    }
}

// ops - Idx

impl<'m, 'd> Add<Idx<'m, 'd>> for Int<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: Idx<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Sca<'m, 'd>.
        let (a, b) = unsafe { (RefPtr::new(self.fun_ptr()), ValPtr::new(rhs.val_ptr())) };
        let f = move || a.val().eval() + b.val();
        self.m.add_int(IntFun(Box::new(f)))
    }
}

impl<'m, 'd> Add<Int<'m, 'd>> for Idx<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: Int<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Sca<'m, 'd>.
        let (a, b) = unsafe { (ValPtr::new(self.val_ptr()), RefPtr::new(rhs.fun_ptr())) };
        let f = move || a.val() + b.val().eval();
        self.m.add_int(IntFun(Box::new(f)))
    }
}

// ops - Idx

impl<'m, 'd> Add<Idx<'m, 'd>> for ParInt<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: Idx<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Sca<'m, 'd>.

        let (a, b) = unsafe { (ValPtr::new(self.val_ptr()), ValPtr::new(rhs.val_ptr())) };
        let f = move || a.val() + b.val();
        self.m.add_int(IntFun(Box::new(f)))
    }
}

impl<'m, 'd> Add<ParInt<'m, 'd>> for Idx<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: ParInt<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Sca<'m, 'd>.
        let (a, b) = unsafe { (ValPtr::new(self.val_ptr()), ValPtr::new(rhs.val_ptr())) };
        let f = move || a.val() + b.val();
        self.m.add_int(IntFun(Box::new(f)))
    }
}

// ops - usize

impl<'m, 'd> Add<usize> for Int<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, b: usize) -> Self::Output {
        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Sca<'m, 'd>.
        let a = unsafe { RefPtr::new(self.fun_ptr()) };
        let f = move || a.val().eval() + b;
        self.m.add_int(IntFun(Box::new(f)))
    }
}

impl<'m, 'd> Add<Int<'m, 'd>> for usize {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: Int<'m, 'd>) -> Self::Output {
        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Sca<'m, 'd>.
        let b = unsafe { RefPtr::new(rhs.fun_ptr()) };
        let f = move || self + b.val().eval();
        rhs.m.add_int(IntFun(Box::new(f)))
    }
}
