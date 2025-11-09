use crate::model::Model;
use crate::symbols::{symbol::Symbol, symbol_meta::SymbolMeta, symbol_ref_core::SymbolRefCore};
use orx_imp_vec::*;

pub struct SymbolDataCollection<S: SymbolMeta> {
    data_vec: ImpVec<Symbol<S>>,
}

impl<S: SymbolMeta> Default for SymbolDataCollection<S> {
    fn default() -> Self {
        Self {
            data_vec: Default::default(),
        }
    }
}

impl<S: SymbolMeta> SymbolDataCollection<S> {
    pub fn push<'m>(&'m self, model: &'m Model, symbol_data: Symbol<S>) -> S::Ref<'m> {
        let data = self.data_vec.imp_push_get_ref(symbol_data);
        let symbol_ref = SymbolRefCore {
            model,
            symbol: data,
        };
        symbol_ref.into()
    }

    pub fn at<'m>(&'m self, model: &'m Model, idx: usize) -> Option<SymbolRefCore<'m, S>> {
        let data = self.data_vec.get(idx)?;
        Some(SymbolRefCore {
            model,
            symbol: data,
        })
    }

    pub fn by_key<'m>(&'m self, model: &'m Model, key: &str) -> Option<SymbolRefCore<'m, S>> {
        let data = self.data_vec.iter().find(|x| x.key.eq(key))?;
        Some(SymbolRefCore {
            model,
            symbol: data,
        })
    }

    pub fn index_of(&self, symbol: SymbolRefCore<'_, S>) -> Option<usize> {
        self.data_vec.index_of(symbol.symbol)
    }
}

impl<'a, S: SymbolMeta> IntoIterator for &'a SymbolDataCollection<S> {
    type Item = &'a Symbol<S>;

    type IntoIter = <&'a SplitVec<Symbol<S>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data_vec.iter()
    }
}

impl<S: SymbolMeta> IntoIterator for SymbolDataCollection<S> {
    type Item = Symbol<S>;

    type IntoIter = <SplitVec<Symbol<S>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data_vec.into_iter()
    }
}
