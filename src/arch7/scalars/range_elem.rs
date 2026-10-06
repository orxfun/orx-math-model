use crate::pars::IntPar0;
use crate::symbol::Symbol;
use crate::Model;
use alloc::boxed::Box;

#[derive(derive_new::new, Clone, Copy)]
pub struct RangeElem<'a>(pub(crate) Symbol<'a, RangeElemData>);

#[derive(derive_new::new)]
pub struct RangeElemData {
    value: Box<dyn Fn(&Model) -> i64>,
}

impl RangeElemData {
    pub fn value(&self, m: &Model) -> i64 {
        (self.value)(m)
    }
}

// from

impl<'a> From<IntPar0<'a>> for RangeElem<'a> {
    fn from(x: IntPar0<'a>) -> Self {
        let pos = x.0.pos();
        let value = Box::new(move |m: &Model| m.int_par0[pos].value());
        // x.0.m.int(value)
        todo!()
    }
}
