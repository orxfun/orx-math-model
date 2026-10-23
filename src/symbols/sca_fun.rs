use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

#[derive(derive_new::new)]
pub struct ScaFun<'d>(Box<dyn Fn() -> f64 + 'd>);

#[derive(derive_new::new, Clone, Copy)]
pub struct Sca<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, ScaFun<'d>>,
}
