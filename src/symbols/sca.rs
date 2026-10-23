use crate::Model;
use alloc::boxed::Box;
use core::ops::Add;
use orx_imp_vec::Elem;

#[derive(derive_new::new)]
pub struct ScaFun<'d>(Box<dyn Fn() -> f64 + 'd>);

impl From<f64> for ScaFun<'_> {
    fn from(value: f64) -> Self {
        Self(Box::new(move || value))
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct Sca<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, ScaFun<'d>>,
}

// ops

impl<'m, 'd> Add<Sca<'m, 'd>> for Sca<'m, 'd> {
    type Output = Sca<'m, 'd>;

    fn add(self, rhs: Sca<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        todo!()
    }
}
