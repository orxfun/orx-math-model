use crate::{
    model::Model,
    stages::Stage,
    symbols::{symbol::Symbol, symbol_meta::SymbolMeta},
};
use alloc::string::String;

pub struct SymbolRefCore<'m, S, M>
where
    S: Stage,
    M: SymbolMeta,
{
    pub model: &'m Model,
    pub symbol: &'m Symbol<S, M>,
}

impl<'m, S, M> Clone for SymbolRefCore<'m, S, M>
where
    S: Stage,
    M: SymbolMeta,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'m, S, M> Copy for SymbolRefCore<'m, S, M>
where
    S: Stage,
    M: SymbolMeta,
{
}

impl<'m, S, M> SymbolRefCore<'m, S, M>
where
    S: Stage,
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

impl<S: Stage, M: SymbolMeta> PartialEq for SymbolRefCore<'_, S, M> {
    fn eq(&self, other: &Self) -> bool {
        core::ptr::addr_eq(self.symbol, other.symbol)
    }
}

impl<S: Stage, M: SymbolMeta> Eq for SymbolRefCore<'_, S, M> {}
