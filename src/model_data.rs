use crate::symbols::{SetSymbol, SymbolDataCollection};

#[derive(Default)]
pub struct ModelData {
    sets: SymbolDataCollection<SetSymbol>,
}
