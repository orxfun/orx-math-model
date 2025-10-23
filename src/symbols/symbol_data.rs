use orx_imp_vec::ImpVec;

pub trait SymbolData {}

pub struct SymbolDataCollection<D: SymbolData> {
    storage: ImpVec<D>,
}

impl<D: SymbolData> SymbolDataCollection<D> {
    pub fn push(&self) {}
}
