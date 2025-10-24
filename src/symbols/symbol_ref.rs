use crate::{
    model::Model,
    symbols::{symbol::Symbol, symbol_data::SymbolData},
};
use alloc::string::String;

pub struct SymbolRef<'m, S>
where
    S: Symbol,
{
    pub(crate) model: &'m Model,
    pub(crate) data: &'m SymbolData<S>,
}

impl<'m, S> Clone for SymbolRef<'m, S>
where
    S: Symbol,
{
    fn clone(&self) -> Self {
        Self {
            model: self.model,
            data: self.data,
        }
    }
}

impl<'m, S> Copy for SymbolRef<'m, S> where S: Symbol {}

impl<'m, S> SymbolRef<'m, S>
where
    S: Symbol,
{
    // pub(crate) fn new(model: &'m Model, data: &'m SymbolData<Data>) -> Self {
    //     Self { model, data }
    // }

    pub fn definition(self, definition: impl Into<String>) -> Self {
        self.data.definition.set(definition);
        self
    }
}
