use crate::{
    model::Model,
    symbols::{symbol::Symbol, symbol_data::SymbolData},
};
use alloc::string::String;
#[cfg(feature = "std")]
use core::hash::Hash;
use core::ptr::addr_of;

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

// ref by hash

#[cfg(feature = "std")]
impl<S: Symbol> Hash for SymbolRef<'_, S> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        addr_of!(self.data).hash(state);
    }
}

// ordering by address

impl<S: Symbol> PartialOrd for SymbolRef<'_, S> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        addr_of!(self.data).partial_cmp(&addr_of!(other.data))
    }
}

impl<S: Symbol> Ord for SymbolRef<'_, S> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        addr_of!(self.data).cmp(&addr_of!(other.data))
    }
}
