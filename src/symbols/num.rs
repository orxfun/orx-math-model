use crate::ptr::RefPtr;
use crate::symbols::{Int, Par, ParData};
use crate::Model;
use alloc::boxed::Box;
use core::ops::Add;
use orx_imp_vec::Elem;

#[derive(derive_new::new)]
pub struct NumFun<'d>(Box<dyn Fn() -> f64 + 'd>);

impl NumFun<'_> {
    pub fn eval(&self) -> f64 {
        self.0()
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct Num<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, NumFun<'d>>,
}

impl<'m, 'd> From<Par<'m, 'd>> for Num<'m, 'd> {
    fn from(value: Par<'m, 'd>) -> Self {
        let p = ParData::ptr_value(value.el.as_ptr());
        let fun = NumFun(Box::new(move || unsafe { *p }));
        value.m.add_num(fun)
    }
}

// ref

struct PtrNumFun<'d>(*const NumFun<'d>);

impl<'d> PtrNumFun<'d> {
    /// # SAFETY
    ///
    /// * Created value must not outlive `'m` or `'d`.
    unsafe fn new<'m>(set: Num<'m, 'd>) -> Self {
        Self(unsafe { set.el.get() })
    }

    fn fun(&self) -> &NumFun<'d> {
        unsafe { &*self.0 }
    }
}

// ops - Num

impl<'m, 'd> Add<Num<'m, 'd>> for Num<'m, 'd> {
    type Output = Num<'m, 'd>;

    fn add(self, rhs: Num<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Sca<'m, 'd>.
        let [a, b] = unsafe { [PtrNumFun::new(self), PtrNumFun::new(rhs)] };
        let f = move || a.fun().eval() + b.fun().eval();
        self.m.add_num(NumFun(Box::new(f)))
    }
}

// ops - f64

impl<'m, 'd> Add<f64> for Num<'m, 'd> {
    type Output = Num<'m, 'd>;

    fn add(self, b: f64) -> Self::Output {
        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Sca<'m, 'd>.
        let a = unsafe { PtrNumFun::new(self) };
        let f = move || a.fun().eval() + b;
        self.m.add_num(NumFun(Box::new(f)))
    }
}

impl<'m, 'd> Add<Num<'m, 'd>> for f64 {
    type Output = Num<'m, 'd>;

    fn add(self, rhs: Num<'m, 'd>) -> Self::Output {
        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Sca<'m, 'd>.
        let b = unsafe { PtrNumFun::new(rhs) };
        let f = move || self + b.fun().eval();
        rhs.m.add_num(NumFun(Box::new(f)))
    }
}

// ops - usize

impl<'m, 'd> Add<usize> for Num<'m, 'd> {
    type Output = Num<'m, 'd>;

    fn add(self, b: usize) -> Self::Output {
        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Sca<'m, 'd>.
        let a = unsafe { PtrNumFun::new(self) };
        let f = move || a.fun().eval() + b as f64;
        self.m.add_num(NumFun(Box::new(f)))
    }
}

impl<'m, 'd> Add<Num<'m, 'd>> for usize {
    type Output = Num<'m, 'd>;

    fn add(self, rhs: Num<'m, 'd>) -> Self::Output {
        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Sca<'m, 'd>.
        let b = unsafe { PtrNumFun::new(rhs) };
        let f = move || self as f64 + b.fun().eval();
        rhs.m.add_num(NumFun(Box::new(f)))
    }
}

// ops - Int

impl<'m, 'd> Add<Int<'m, 'd>> for Num<'m, 'd> {
    type Output = Num<'m, 'd>;

    fn add(self, rhs: Int<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Sca<'m, 'd>.
        let (a, b) = unsafe { (PtrNumFun::new(self), RefPtr::new(rhs.fun_ptr())) };
        let f = move || a.fun().eval() + b.val().eval() as f64;
        self.m.add_num(NumFun(Box::new(f)))
    }
}

impl<'m, 'd> Add<Num<'m, 'd>> for Int<'m, 'd> {
    type Output = Num<'m, 'd>;

    fn add(self, rhs: Num<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Sca<'m, 'd>.
        let (a, b) = unsafe { (RefPtr::new(self.fun_ptr()), PtrNumFun::new(rhs)) };
        let f = move || a.val().eval() as f64 + b.fun().eval();
        self.m.add_num(NumFun(Box::new(f)))
    }
}
