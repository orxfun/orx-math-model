use crate::symbols::symbol::Symbol;
use orx_imp_vec::ImpVec;

pub struct SymbolDataCollection<S: Symbol> {
    data_vec: ImpVec<S::Data>,
}

impl<S: Symbol> SymbolDataCollection<S> {
    pub fn push(&self, data: S::Data) {
        //
    }
}
