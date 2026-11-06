use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// data

#[derive(derive_new::new)]
pub struct DVal<'d, T> {
    fun: Box<dyn Fn() -> T + 'd>,
}

// sym

#[derive(derive_new::new)]
#[new(visibility = "pub(crate)")]
pub struct Val<'m, 'd, T> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, DVal<'d, T>>,
}
