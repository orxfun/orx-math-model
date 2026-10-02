use crate::symbols::{Model, SymbolDefinition};

#[derive(derive_new::new, Clone, Copy)]
pub struct RangeSet<'a> {
    m: &'a Model,
    d: &'a SymbolDefinition,
}
