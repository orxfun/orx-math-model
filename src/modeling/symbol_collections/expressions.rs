use super::symbol_collection::SymbolCollection;
use crate::{
    modeling::stages::{Building, Built, ModelStages},
    modeling::{
        model_data::ModelData,
        reference::{HasRef, SymRef},
        symbols::{data::expression_data::ExpressionData, expression::Expression},
    },
};
use orx_imp_vec::ImpVec;
use std::rc::Rc;

#[derive(Default)]
pub(crate) struct Expressions<B: ModelStages> {
    storage: ImpVec<ExpressionData<B>>,
}

impl<B: ModelStages> SymbolCollection for Expressions<B> {
    type Storage = ExpressionData<B>;
    type Symbol = Expression<B>;
    fn storage(&self) -> &ImpVec<Self::Storage> {
        &self.storage
    }
}

impl<B: ModelStages> Expressions<B> {
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
            .chain(
                self.storage
                    .iter()
                    .flat_map(|x| x.constant_scalar_ref.iter()),
            )
            .chain(self.storage.iter().flat_map(|x| x.term_refs.iter()))
            .chain(self.storage.iter().flat_map(|x| x.sum_refs.iter()))
    }
}

impl Expressions<Building> {
    pub(crate) fn add_to_built_model(&self, model: &Rc<ModelData<Built>>) {
        for d in &self.storage {
            model.expressions.storage.push(d.build(model));
        }
    }
}
