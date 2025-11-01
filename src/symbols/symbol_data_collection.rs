use crate::{
    model::Model,
    symbols::{symbol::Symbol, symbol_data::SymbolData, symbol_ref_core::SymbolRefCore},
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
        let symbol_ref = SymbolRefCore { model, data };
        symbol_ref.into()
    }

    pub fn at<'m>(&'m self, model: &'m Model, idx: usize) -> Option<SymbolRefCore<'m, S>> {
        let data = self.data_vec.get(idx)?;
        Some(SymbolRefCore { model, data })
    }

    pub fn by_key<'m>(&'m self, model: &'m Model, key: &str) -> Option<SymbolRefCore<'m, S>> {
        let data = self.data_vec.iter().find(|x| x.key.eq(key))?;
        Some(SymbolRefCore { model, data })
    }

    pub fn index_of(&self, symbol: SymbolRefCore<'_, S>) -> Option<usize> {
        self.data_vec.index_of(symbol.data)
    }
}
