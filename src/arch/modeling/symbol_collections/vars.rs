use std::rc::Rc;

use super::symbol_collection::SymbolCollection;
use crate::{
    modeling::stages::{Building, Built, ModelStages},
    modeling::{
        model_data::ModelData,
        reference::{HasRef, SymRef},
        symbols::{data::var_data::VarData, var::Var},
    },
};
use orx_imp_vec::ImpVec;

pub(crate) struct Vars<B: ModelStages> {
    storage: ImpVec<VarData<B>>,
}

impl<B: ModelStages> SymbolCollection for Vars<B> {
    type Storage = VarData<B>;
    type Symbol = Var<B>;
    fn storage(&self) -> &ImpVec<Self::Storage> {
        &self.storage
    }
}

impl<B: ModelStages> Vars<B> {
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
            .chain(self.storage.iter().map(|x| &x.variable_ref))
            .chain(self.storage.iter().flat_map(|x| x.scalar_refs.as_slice()))
    }
}

impl Vars<Building> {
    pub(crate) fn add_to_built_model(&self, model: &Rc<ModelData<Built>>) {
        for d in &self.storage {
            model.vars.storage.push(d.build(model));
        }
    }
}
