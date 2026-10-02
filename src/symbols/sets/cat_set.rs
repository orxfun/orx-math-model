use crate::symbols::{Model, SymbolData};

pub struct CatSet<'a> {
    m: &'a Model,
    d: &'a SymbolData,
}
