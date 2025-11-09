use crate::no_std_types::Map;
use crate::symbols::sets::{SetCore, SetMeta};
use crate::symbols::Symbol;
use crate::symbols::{symbol_meta::SymbolMeta, symbol_ref_core::SymbolRefCore};
use core::marker::PhantomData;

pub type SetCoreMap<'m, V> = SymbolMap<'m, SetMeta, SetCore<'m>, V>;

pub struct SymbolMap<'m, S, K, V>
where
    S: SymbolMeta,
    K: Into<SymbolRefCore<'m, S>>,
{
    map: Map<usize, V>,
    phantom: PhantomData<(K, SymbolRefCore<'m, S>)>,
}

impl<'m, S, K, V> Default for SymbolMap<'m, S, K, V>
where
    S: SymbolMeta,
    K: Into<SymbolRefCore<'m, S>>,
{
    fn default() -> Self {
        Self {
            map: Map::default(),
            phantom: PhantomData,
        }
    }
}

impl<'m, S, K, V> SymbolMap<'m, S, K, V>
where
    S: SymbolMeta,
    K: Into<SymbolRefCore<'m, S>>,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, symbol: K, value: V) {
        self.map.insert(Self::key_of(symbol), value);
    }

    pub fn try_insert(&mut self, symbol: K, value: V) -> bool {
        let key = Self::key_of(symbol);
        match self.contains_key(key) {
            true => false,
            false => {
                self.map.insert(key, value);
                true
            }
        }
    }

    pub fn get(&self, symbol: K) -> Option<&V> {
        self.map.get(&Self::key_of(symbol))
    }

    pub fn contains_key(&self, key: usize) -> bool {
        self.map.contains_key(&key)
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[cfg(test)]
    pub fn contains_symbol(&self, symbol: K) -> bool {
        self.contains_key(Self::key_of(symbol))
    }

    // helper

    #[inline(always)]
    fn key_of(symbol: K) -> usize {
        let symbol = symbol.into();
        Symbol::addr(symbol.symbol)
    }
}
