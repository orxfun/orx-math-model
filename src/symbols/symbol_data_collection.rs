use crate::model::Model;
use crate::stages::Stage;
use crate::symbols::{symbol::Symbol, symbol_meta::SymbolMeta, symbol_ref_core::SymbolRefCore};
use orx_imp_vec::*;

pub struct SymbolDataCollection<S: Stage, M: SymbolMeta> {
    data_vec: ImpVec<Symbol<S, M>>,
}

impl<S: Stage, M: SymbolMeta> Default for SymbolDataCollection<S, M> {
    fn default() -> Self {
        Self {
            data_vec: Default::default(),
        }
    }
}

impl<S: Stage, M: SymbolMeta> SymbolDataCollection<S, M> {
    pub fn push<'m>(&'m self, model: &'m Model, symbol_data: Symbol<S, M>) -> M::Ref<'m> {
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
