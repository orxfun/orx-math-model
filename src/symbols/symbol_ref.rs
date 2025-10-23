use crate::model::Model;

pub struct SymbolRef<'m, Data> {
    model: &'m Model,
    data: &'m Data,
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
    pub fn new(model: &'m Model, data: &'m Data) -> Self {
        Self { model, data }
    }
}
