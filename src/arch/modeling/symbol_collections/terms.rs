use super::symbol_collection::SymbolCollection;
use crate::{
    modeling::stages::{Building, Built, ModelStages},
    modeling::{
        model_data::ModelData,
        reference::{HasRef, SymRef},
        symbols::{data::term_data::TermData, term::Term},
    },
};
use orx_imp_vec::ImpVec;
use std::rc::Rc;

#[derive(Default)]
pub(crate) struct Terms<B: ModelStages> {
    storage: ImpVec<TermData<B>>,
}
impl<B: ModelStages> SymbolCollection for Terms<B> {
    type Storage = TermData<B>;
    type Symbol = Term<B>;
    fn storage(&self) -> &ImpVec<Self::Storage> {
        &self.storage
    }
}

impl<B: ModelStages> Terms<B> {
    pub(crate) fn new() -> Self {
        let storage = ImpVec::default();
        Self { storage }
    }
    pub(crate) fn iter_debug(&self) -> impl Iterator<Item = String> + '_ {
        self.storage.iter().map(|x| format!("{:?}", x.symbol))
    }
    pub(crate) fn all_model_references(&self) -> impl Iterator<Item = &SymRef<B>> {
        self.storage
            .iter()
            .map(|x| x.symbol.sym_ref())
            .chain(self.storage.iter().map(|x| &x.coef_scalar_ref))
            .chain(self.storage.iter().map(|x| &x.var_ref))
    }
}

impl Terms<Building> {
    pub(crate) fn add_to_built_model(&self, model: &Rc<ModelData<Built>>) {
        for d in &self.storage {
            model.terms.storage.push(d.build(model));
        }
    }
}
