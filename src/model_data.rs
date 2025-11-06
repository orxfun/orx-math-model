use crate::symbols::params::ParMeta;
use crate::symbols::sets::SetMeta;
use crate::symbols::SymbolDataCollection;

#[derive(Default)]
pub struct ModelData {
    pub(super) sets: SymbolDataCollection<SetMeta>,
    pub(super) pars: SymbolDataCollection<ParMeta>,
}
