use super::symbol_collection::SymbolCollection;
use crate::{
    modeling::stages::{Building, Built, ModelStages},
    modeling::{
        model_data::ModelData,
        reference::{HasRef, SymRef},
        symbols::{data::objective_data::ObjectiveData, objective::Objective},
    },
};
use orx_imp_vec::ImpVec;
use std::rc::Rc;

#[derive(Default)]
pub(crate) struct Objectives<B: ModelStages> {
    storage: ImpVec<ObjectiveData<B>>,
}

impl<B: ModelStages> SymbolCollection for Objectives<B> {
    type Storage = ObjectiveData<B>;
    type Symbol = Objective<B>;
    fn storage(&self) -> &ImpVec<Self::Storage> {
        &self.storage
    }
}

impl<B: ModelStages> Objectives<B> {
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
            .chain(self.storage.iter().map(|x| &x.expression_ref))
    }
}

impl Objectives<Building> {
    pub(crate) fn add_to_built_model(&self, model: &Rc<ModelData<Built>>) {
        for d in &self.storage {
            model.objectives.storage.push(d.build(model));
        }
    }
}
