use crate::symbols::{SetMeta, SymbolDataCollection};

#[derive(Default)]
pub struct ModelData {
    pub(super) sets: SymbolDataCollection<SetMeta>,
}
