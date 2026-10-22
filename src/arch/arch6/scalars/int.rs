use crate::pars::{IntPar0, IntPar0Data};
use crate::symbol::Symbol;
use alloc::boxed::Box;

#[derive(derive_new::new, Clone, Copy)]
pub struct Int<'a>(pub(crate) Symbol<'a, IntData>);

#[derive(derive_new::new)]
pub struct IntData {
    value: Box<dyn Fn() -> i64>,
}

// from

impl<'a> From<IntPar0<'a>> for Int<'a> {
    fn from(value: IntPar0<'a>) -> Self {
        let (m, d) = value.0.destruct();
        let value = Box::new(|| d.value);
        m.int(value)
    }
}
