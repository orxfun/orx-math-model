use crate::{
    model::Model,
    symbols::{symbol_data::SymbolData, symbol_definition::SymbolDef},
};
use alloc::string::String;

pub struct SymbolRef<'m, S>
where
    S: SymbolDef,
{
    pub(crate) model: &'m Model,
    pub(crate) data: &'m SymbolData<S>,
}

impl<'m, S> Clone for SymbolRef<'m, S>
where
    S: SymbolDef,
{
    fn clone(&self) -> Self {
        Self {
            model: self.model,
            data: self.data,
        }
    }
}

impl<'m, S> Copy for SymbolRef<'m, S> where S: SymbolDef {}

impl<'m, S> SymbolRef<'m, S>
where
    S: SymbolDef,
{
    // pub(crate) fn new(model: &'m Model, data: &'m SymbolData<Data>) -> Self {
    //     Self { model, data }
    // }

    pub fn definition(self, definition: impl Into<String>) -> Self {
        self.data.definition.set(definition);
        self
    }
}
