use super::symbol::SymbolData;
use orx_imp_vec::ImpVec;

pub struct SymbolDataCollection<S: SymbolData> {
    storage: ImpVec<S>,
}

impl<S: SymbolData> SymbolDataCollection<S> {}
