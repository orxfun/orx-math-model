use crate::symbols::{Model, SymbolDefinition};

#[derive(derive_new::new, Clone, Copy)]
pub struct Symbol<'a> {
    pub m: &'a Model,
    pub d: &'a SymbolDefinition,
}
