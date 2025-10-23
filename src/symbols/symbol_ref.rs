use crate::{model::Model, symbols::symbol_data::SymbolData};
use alloc::string::String;

pub struct SymbolRef<'m, Data> {
    pub(crate) model: &'m Model,
    pub(crate) data: &'m SymbolData<Data>,
}

impl<'m, Data> Clone for SymbolRef<'m, Data> {
    fn clone(&self) -> Self {
        Self {
            model: self.model,
            data: self.data,
        }
    }
}

impl<'m, Data> Copy for SymbolRef<'m, Data> {}

impl<'m, Data> SymbolRef<'m, Data> {
    // pub(crate) fn new(model: &'m Model, data: &'m SymbolData<Data>) -> Self {
    //     Self { model, data }
    // }

    pub fn definition(self, definition: impl Into<String>) -> Self {
        self.data.definition.set(definition);
        self
    }
}
