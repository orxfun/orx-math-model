use crate::{
    no_std_types::Map,
    symbols::{symbol::Symbol, symbol_ref::SymbolRef, SymbolData},
};
use core::marker::PhantomData;

pub struct SymbolMap<'m, S, K, V>
where
    S: Symbol,
    K: Into<SymbolRef<'m, S>>,
{
    map: Map<usize, V>,
    phantom: PhantomData<(K, SymbolRef<'m, S>)>,
}

impl<'m, S, K, V> SymbolMap<'m, S, K, V>
where
    S: Symbol,
    K: Into<SymbolRef<'m, S>>,
{
    pub fn new() -> Self {
        Self {
            map: Map::new(),
            phantom: PhantomData,
        }
    }

    pub fn insert(&mut self, symbol: K, value: V) {
        self.map.insert(Self::addr_of(symbol), value);
    }

    pub fn get(&self, symbol: K) -> Option<&V> {
        self.map.get(&Self::addr_of(symbol))
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn contains_key(&self, symbol: K) -> bool {
        self.map.contains_key(&Self::addr_of(symbol))
    }

    // helper

    #[inline(always)]
    fn addr_of(symbol: K) -> usize {
        let symbol = symbol.into();
        symbol.data as *const SymbolData<S> as usize
    }
}
