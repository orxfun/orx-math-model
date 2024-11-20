use std::rc::Rc;

use super::symbol_collection::SymbolCollection;
use crate::{
    modeling::stages::{Building, Built, ModelStages},
    modeling::{
        model_data::ModelData,
        reference::{HasRef, SymRef},
        symbols::{data::sum_data::SumData, sum::Sum},
    },
};
use orx_imp_vec::ImpVec;

#[derive(Default)]
pub(crate) struct Sums<B: ModelStages> {
    storage: ImpVec<SumData<B>>,
}
impl<B: ModelStages> SymbolCollection for Sums<B> {
    type Storage = SumData<B>;
    type Symbol = Sum<B>;
    fn storage(&self) -> &ImpVec<Self::Storage> {
        &self.storage
    }
}

impl<B: ModelStages> Sums<B> {
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
            .chain(self.storage.iter().flat_map(|x| x.sum_over_set_refs.iter()))
            .chain(self.storage.iter().map(|x| &x.expression_ref))
    }
}

impl Sums<Building> {
    pub(crate) fn add_to_built_model(&self, model: &Rc<ModelData<Built>>) {
        for d in &self.storage {
            model.sums.storage.push(d.build(model));
        }
    }
}
