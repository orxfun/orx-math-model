use crate::symbols::{Model, SymbolData};

#[derive(derive_new::new, Clone, Copy)]
pub struct NumSet<'a> {
    m: &'a Model,
    d: &'a SymbolData,
}
