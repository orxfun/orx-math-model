use orx_imp_vec::ImpVec;

pub trait SymbolData {}

pub struct SymbolDataCollection<S: SymbolData> {
    storage: ImpVec<S>,
}

impl<S: SymbolData> SymbolDataCollection<S> {}
