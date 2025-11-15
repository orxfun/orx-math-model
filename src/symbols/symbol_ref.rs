use crate::model::Model;

pub struct SymbolRef<'m, S> {
    pub model_ref: &'m Model,
    pub data_ref: &'m S,
}

impl<'m, S> Clone for SymbolRef<'m, S> {
    fn clone(&self) -> Self {
        Self {
            model_ref: self.model_ref,
            data_ref: self.data_ref,
        }
    }
}

impl<'m, S> Copy for SymbolRef<'m, S> {}
