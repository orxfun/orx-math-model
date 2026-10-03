use crate::symbols::{Model, Symbol, SymbolDefinition};

#[derive(derive_new::new, Clone, Copy)]
pub struct CatSet<'a> {
    m: &'a Model,
    d: &'a CatSetData,
}

#[derive(derive_new::new)]
pub struct CatSetData {
    pub def: SymbolDefinition,
    pub elem_pos: usize,
}
