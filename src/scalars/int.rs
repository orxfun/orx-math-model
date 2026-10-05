use crate::model::Model;
use crate::pars::{IntPar0, IntPar0Data};
use crate::symbol::Symbol;
use alloc::boxed::Box;

#[derive(derive_new::new, Clone, Copy)]
pub struct Int<'a>(pub(crate) Symbol<'a, IntData>);

#[derive(derive_new::new)]
pub struct IntData {
    value: Box<dyn Fn(&Model) -> i64>,
}

// from

impl<'a> From<IntPar0<'a>> for Int<'a> {
    fn from(x: IntPar0<'a>) -> Self {
        let pos = x.0.pos();
        let value = Box::new(move |m: &Model| m.int_par0[pos].value);
        x.0.m.int(value)
    }
}
