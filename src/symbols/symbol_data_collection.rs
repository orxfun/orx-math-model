use crate::{
    model::Model,
    symbols::{
        definition::Definition, symbol::Symbol, symbol_data::SymbolData, symbol_ref::SymbolRef,
    },
};
use alloc::string::String;
use orx_imp_vec::ImpVec;

pub struct SymbolDataCollection<S: Symbol> {
    data_vec: ImpVec<SymbolData<S::Data>>,
}

impl<S: Symbol> Default for SymbolDataCollection<S> {
    fn default() -> Self {
        Self {
            data_vec: Default::default(),
        }
    }
}

impl<S: Symbol> SymbolDataCollection<S> {
    pub fn push<'m>(&'m self, model: &'m Model, symbol_data: SymbolData<S::Data>) -> S::Ref<'m> {
        let data_ref = self.data_vec.imp_push_get_ref(symbol_data);
        SymbolRef::new(model, data_ref).into()
    }
}
