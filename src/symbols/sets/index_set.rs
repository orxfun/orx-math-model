use crate::symbols::{Model, SymbolData};

#[derive(derive_new::new, Clone, Copy)]
pub struct IndexSet<'a> {
    m: &'a Model,
    d: &'a SymbolData,
}
