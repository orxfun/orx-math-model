use crate::{
    model::Model,
    symbols::{symbol::Symbol, symbol_meta::SymbolMeta},
};
use alloc::string::String;

pub struct SymbolRefCore<'m, M>
where
    M: SymbolMeta,
{
    pub model: &'m Model,
    pub symbol: &'m Symbol<M>,
}

impl<'m, M> Clone for SymbolRefCore<'m, M>
where
    M: SymbolMeta,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'m, M> Copy for SymbolRefCore<'m, M> where M: SymbolMeta {}

impl<'m, M> SymbolRefCore<'m, M>
where
    M: SymbolMeta,
{
    pub fn key(self, key: impl Into<String>) -> Self {
        self.symbol.key.set(key);
        self
    }

    pub fn definition(self, definition: impl Into<String>) -> Self {
        self.symbol.definition.set(definition);
        self
    }
}

// reference equality

impl<M: SymbolMeta> PartialEq for SymbolRefCore<'_, M> {
    fn eq(&self, other: &Self) -> bool {
        core::ptr::addr_eq(self.symbol, other.symbol)
    }
}

impl<M: SymbolMeta> Eq for SymbolRefCore<'_, M> {}
