use crate::{
    model::Model,
    symbols::{symbol::Symbol, symbol_ref::SymbolRef},
};
use orx_imp_vec::ImpVec;

pub struct SymbolDataCollection<S: Symbol> {
    data_vec: ImpVec<S::Data>,
}

impl<S: Symbol> SymbolDataCollection<S> {
    pub fn push<'m>(&'m self, model: &'m Model, data: S::Data) -> S::Ref<'m> {
        let data_ref = self.data_vec.imp_push_get_ref(data);
        SymbolRef::new(model, data_ref).into()
    }
}
