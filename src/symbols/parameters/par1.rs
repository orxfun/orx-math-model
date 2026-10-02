use crate::symbols::{sets::Set, Model, SymbolDefinition};

#[derive(derive_new::new, Clone, Copy)]
pub struct Par1<'a> {
    m: &'a Model,
    d: &'a SymbolDefinition,
    i: Set<'a>,
}

#[derive(derive_new::new, Clone, Copy)]
pub struct IntPar1<'a> {
    m: &'a Model,
    d: &'a SymbolDefinition,
    i: Set<'a>,
}
