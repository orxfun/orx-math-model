use crate::symbols::{parameters::IntPar0, Model, Symbol, SymbolDefinition};
use alloc::boxed::Box;

#[derive(derive_new::new, Clone, Copy)]
pub struct Int<'a> {
    pub m: &'a Model,
    pub d: &'a IntData,
}

#[derive(derive_new::new)]
pub struct IntData {
    pos: usize,
    def: SymbolDefinition,
    create: Box<dyn Fn(&Model) -> i64>,
}

// from

impl<'a> From<IntPar0<'a>> for Int<'a> {
    fn from(value: IntPar0<'a>) -> Self {
        let (m, pos) = (value.m, value.pos);
        let create = Box::new(move |m: &Model| m.pars.int_par0[pos].value);
        let def = value.d.def.clone();
        m.scalars.add_int(m, def, create)
    }
}
