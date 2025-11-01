use crate::model::Model;
use crate::symbols::{symbol::Symbol, symbol_meta::SymbolMeta, symbol_ref_core::SymbolRefCore};
use orx_imp_vec::*;

pub struct SymbolDataCollection<M: SymbolMeta> {
    data_vec: ImpVec<Symbol<M>>,
}

impl<M: SymbolMeta> Default for SymbolDataCollection<M> {
    fn default() -> Self {
        Self {
            data_vec: Default::default(),
        }
    }
}

impl<M: SymbolMeta> SymbolDataCollection<M> {
    pub fn push<'m>(&'m self, model: &'m Model, symbol_data: Symbol<M>) -> M::Ref<'m> {
        let data = self.data_vec.imp_push_get_ref(symbol_data);
        let symbol_ref = SymbolRefCore {
            model,
            symbol: data,
        };
        symbol_ref.into()
    }

    pub fn at<'m>(&'m self, model: &'m Model, idx: usize) -> Option<SymbolRefCore<'m, M>> {
        let data = self.data_vec.get(idx)?;
        Some(SymbolRefCore {
            model,
            symbol: data,
        })
    }

    pub fn by_key<'m>(&'m self, model: &'m Model, key: &str) -> Option<SymbolRefCore<'m, M>> {
        let data = self.data_vec.iter().find(|x| x.key.eq(key))?;
        Some(SymbolRefCore {
            model,
            symbol: data,
        })
    }

    pub fn index_of(&self, symbol: SymbolRefCore<'_, M>) -> Option<usize> {
        self.data_vec.index_of(symbol.symbol)
    }
}
