use crate::{
    model::Model,
    symbols::{symbol::Symbol, symbol_data::SymbolData},
};
use alloc::string::String;

pub struct SymbolRefCore<'m, S>
where
    S: Symbol,
{
    pub model: &'m Model,
    pub data: &'m SymbolData<S>,
}

impl<'m, S> Clone for SymbolRefCore<'m, S>
where
    S: Symbol,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'m, S> Copy for SymbolRefCore<'m, S> where S: Symbol {}

impl<'m, S> SymbolRefCore<'m, S>
where
    S: Symbol,
{
    pub fn key(self, key: impl Into<String>) -> Self {
        self.data.key.set(key);
        self
    }

    pub fn definition(self, definition: impl Into<String>) -> Self {
        self.data.definition.set(definition);
        self
    }
}

// reference equality

impl<S: Symbol> PartialEq for SymbolRefCore<'_, S> {
    fn eq(&self, other: &Self) -> bool {
        core::ptr::addr_eq(self.data, other.data)
    }
}

impl<S: Symbol> Eq for SymbolRefCore<'_, S> {}
