use crate::{symbols::Par, Model};
use alloc::boxed::Box;
use core::ops::Add;
use orx_imp_vec::Elem;

#[derive(derive_new::new)]
pub struct ScaFun<'d>(Box<dyn Fn() -> f64 + 'd>);

impl ScaFun<'_> {
    pub fn eval(&self) -> f64 {
        self.0()
    }
}

impl From<f64> for ScaFun<'_> {
    fn from(value: f64) -> Self {
        Self(Box::new(move || value))
    }
}

impl<'m, 'd> From<Par<'m, 'd>> for ScaFun<'d> {
    fn from(value: Par<'m, 'd>) -> Self {
        let p = unsafe { value.el.get() }.value() as *const f64;
        Self(Box::new(move || unsafe { *p }))
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct Sca<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, ScaFun<'d>>,
}

// ref

struct RefScaFun<'d>(*const ScaFun<'d>);

impl<'d> RefScaFun<'d> {
    /// # SAFETY
    ///
    /// * Created value must not outlive `'m` or `'d`.
    unsafe fn new<'m>(set: Sca<'m, 'd>) -> Self {
        Self(unsafe { set.el.get() })
    }

    fn fun(&self) -> &ScaFun<'d> {
        unsafe { &*self.0 }
    }
}

// ops

impl<'m, 'd> Add<Sca<'m, 'd>> for Sca<'m, 'd> {
    type Output = Sca<'m, 'd>;

    fn add(self, rhs: Sca<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Sca<'m, 'd>.
        let [a, b] = unsafe { [RefScaFun::new(self), RefScaFun::new(rhs)] };
        let f = move || a.fun().eval() + b.fun().eval();
        self.m.add_sca(ScaFun(Box::new(f)))
    }
}
