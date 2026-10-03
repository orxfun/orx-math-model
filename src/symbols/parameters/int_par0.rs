use crate::symbols::{Model, SymbolDefinition};

#[derive(derive_new::new, Clone, Copy)]
pub struct IntPar0<'a> {
    pub(crate) m: &'a Model,
    pub(crate) d: &'a IntPar0Data,
    pub(crate) pos: usize,
}

#[derive(derive_new::new)]
pub struct IntPar0Data {
    pub def: SymbolDefinition,
    pub value: i64,
}
