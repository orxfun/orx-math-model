use crate::{
    model::Model,
    symbols::{symbol_data::SymbolData, symbol_definition::SymbolDef, symbol_ref::SymbolRef},
};
use orx_imp_vec::ImpVec;

pub struct SymbolDataCollection<S: SymbolDef> {
    data_vec: ImpVec<SymbolData<S>>,
}

impl<S: SymbolDef> Default for SymbolDataCollection<S> {
    fn default() -> Self {
        Self {
            data_vec: Default::default(),
        }
    }
}

impl<S: SymbolDef> SymbolDataCollection<S> {
    pub fn push<'m>(&'m self, model: &'m Model, symbol_data: SymbolData<S>) -> S::Ref<'m> {
        let data = self.data_vec.imp_push_get_ref(symbol_data);
        let symbol_ref = SymbolRef { model, data };
        symbol_ref.into()
    }
}
