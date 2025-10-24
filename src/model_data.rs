use crate::symbols::{SetSymbol, SymbolDataCollection};

#[derive(Default)]
pub struct ModelData {
    pub(super) sets: SymbolDataCollection<SetSymbol>,
}
