use crate::symbols::{parameters::Count, Model, SymbolData};

#[derive(derive_new::new, Clone, Copy)]
pub struct IndexedSet<'a> {
    m: &'a Model,
    d: &'a SymbolData,
    count: Count<'a>,
}
