use crate::symbols::{Model, SymbolData};

pub struct NumSet<'a> {
    m: &'a Model,
    d: &'a SymbolData,
}
