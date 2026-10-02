use crate::symbols::{Model, SymbolDefinition};

#[derive(derive_new::new, Clone, Copy)]
pub struct Symbol<'a> {
    m: &'a Model,
    d: &'a SymbolDefinition,
}
