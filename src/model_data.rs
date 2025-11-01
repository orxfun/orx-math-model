use crate::stages::Stage;
use crate::symbols::{SetMeta, SymbolDataCollection};

#[derive(Default)]
pub struct ModelData<S: Stage> {
    pub(super) sets: SymbolDataCollection<S, SetMeta>,
}
