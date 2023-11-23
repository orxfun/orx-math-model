use super::symbol_collection::SymbolCollection;
use crate::{
    modeling::stages::{Building, Built, ModelStages},
    modeling::{
        model_data::ModelData,
        reference::{HasRef, SymRef},
        symbols::{data::set_data::SetData, set::Set},
    },
};
use orx_imp_vec::ImpVec;
use std::rc::Rc;

pub(crate) struct Sets<B: ModelStages> {
    storage: ImpVec<SetData<B>>,
}

impl<B: ModelStages> SymbolCollection for Sets<B> {
    type Storage = SetData<B>;
    type Symbol = Set<B>;
    fn storage(&self) -> &ImpVec<Self::Storage> {
        &self.storage
    }
}

impl<B: ModelStages> Sets<B> {
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
            .chain(self.storage.iter().map(|x| &x.scalar_ref))
    }
}

impl Sets<Building> {
    pub(crate) fn add_to_built_model(&self, model: &Rc<ModelData<Built>>) {
        for d in &self.storage {
            model.sets.storage.push(d.build(model));
        }
    }
}
