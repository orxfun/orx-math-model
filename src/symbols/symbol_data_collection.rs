use crate::{
    model::Model,
    symbols::{symbol::Symbol, symbol_data::SymbolData, symbol_ref::SymbolRef},
};
use orx_imp_vec::*;

pub struct SymbolDataCollection<S: Symbol> {
    data_vec: ImpVec<SymbolData<S>>,
}

impl<S: Symbol> Default for SymbolDataCollection<S> {
    fn default() -> Self {
        Self {
            data_vec: Default::default(),
        }
    }
}

impl<S: Symbol> SymbolDataCollection<S> {
    pub fn push<'m>(&'m self, model: &'m Model, symbol_data: SymbolData<S>) -> S::Ref<'m> {
        let data = self.data_vec.imp_push_get_ref(symbol_data);
        let symbol_ref = SymbolRef { model, data };
        symbol_ref.into()
    }

    pub fn at<'m>(&'m self, model: &'m Model, idx: usize) -> Option<SymbolRef<'m, S>> {
        let data = self.data_vec.get(idx)?;
        Some(SymbolRef { model, data })
    }

    pub fn index_of(&self, symbol: SymbolRef<'_, S>) -> Option<usize> {
        self.data_vec.index_of(symbol.data)
    }
}
