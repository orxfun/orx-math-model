use crate::symbols::{parameters::Count, Model, SymbolDefinition};

#[derive(derive_new::new, Clone, Copy)]
pub struct IndexedSet<'a> {
    m: &'a Model,
    d: &'a SymbolDefinition,
    count: Count<'a>,
}
