use crate::{
    model::Model,
    symbols::{symbol::Symbol, symbol_meta::SymbolMeta},
};
use alloc::string::String;

pub struct SymbolRefCore<'m, S>
where
    S: SymbolMeta,
{
    pub model: &'m Model,
    pub symbol: &'m Symbol<S>,
}

impl<'m, S> Clone for SymbolRefCore<'m, S>
where
    S: SymbolMeta,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'m, S> Copy for SymbolRefCore<'m, S> where S: SymbolMeta {}

impl<'m, S> SymbolRefCore<'m, S>
where
    S: SymbolMeta,
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

impl<S: SymbolMeta> PartialEq for SymbolRefCore<'_, S> {
    fn eq(&self, other: &Self) -> bool {
        core::ptr::addr_eq(self.symbol, other.symbol)
    }
}

impl<S: SymbolMeta> Eq for SymbolRefCore<'_, S> {}
