use crate::symbols::{parameters::IntPar0, Model, Symbol, SymbolDefinition};
use alloc::boxed::Box;

#[derive(derive_new::new, Clone, Copy)]
pub struct Int<'a> {
    s: Symbol<'a>,
}

#[derive(derive_new::new)]
pub struct IntData {
    d: SymbolDefinition,
    create: Box<dyn Fn(&Model) -> isize>,
}

// from

impl<'a> From<IntPar0<'a>> for Int<'a> {
    fn from(value: IntPar0<'a>) -> Self {
        todo!()
    }
}
