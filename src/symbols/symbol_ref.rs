use crate::{
    model::Model,
    symbols::{symbol::Symbol, symbol_data::SymbolData},
};
use alloc::string::String;

pub struct SymbolRef<'m, S>
where
    S: Symbol,
{
    pub model: &'m Model,
    pub data: &'m SymbolData<S>,
}

impl<'m, S> Clone for SymbolRef<'m, S>
where
    S: Symbol,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'m, S> Copy for SymbolRef<'m, S> where S: Symbol {}

impl<'m, S> SymbolRef<'m, S>
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

impl<S: Symbol> PartialEq for SymbolRef<'_, S> {
    fn eq(&self, other: &Self) -> bool {
        core::ptr::addr_eq(self.data, other.data)
    }
}

impl<S: Symbol> Eq for SymbolRef<'_, S> {}
