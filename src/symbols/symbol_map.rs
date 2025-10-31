#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap;
use core::{marker::PhantomData, ptr::addr_of};
#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(not(feature = "std"))]
type Map<K, V> = BTreeMap<K, V>;
#[cfg(feature = "std")]
type Map<K, V> = HashMap<K, V>;
use crate::symbols::{symbol::Symbol, symbol_ref::SymbolRef};

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
    pub fn insert(&mut self, symbol: impl Into<SymbolRef<'m, S>>, value: V) {
        let symbol = symbol.into();
        let data_addr = addr_of!(symbol.data) as usize;
        self.map.insert(data_addr, value);
    }

    pub fn get(&self, symbol: impl Into<SymbolRef<'m, S>>) -> Option<&V> {
        let symbol = symbol.into();
        let data_addr = addr_of!(symbol.data) as usize;
        self.map.get(&data_addr)
    }
}
