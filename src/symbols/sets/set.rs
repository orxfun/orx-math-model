use crate::symbols::{Model, SymbolDefinition};

#[derive(derive_new::new, Clone, Copy)]
pub struct Set<'a> {
    m: &'a Model,
    d: &'a SymbolDefinition,
}
