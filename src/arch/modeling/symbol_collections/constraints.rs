use super::symbol_collection::SymbolCollection;
use crate::{
    modeling::stages::{Building, Built, ModelStages},
    modeling::{
        model_data::ModelData,
        reference::{HasRef, SymRef},
        symbols::{constraint::Constraint, data::constraint_data::ConstraintData},
    },
};
use orx_imp_vec::ImpVec;
use std::rc::Rc;

#[derive(Default)]
pub(crate) struct Constraints<B: ModelStages> {
    storage: ImpVec<ConstraintData<B>>,
}

impl<B: ModelStages> SymbolCollection for Constraints<B> {
    type Storage = ConstraintData<B>;
    type Symbol = Constraint<B>;
    fn storage(&self) -> &ImpVec<Self::Storage> {
        &self.storage
    }
}

impl<B: ModelStages> Constraints<B> {
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
                    .map(|x| &x.constraint_expression.lhs_expression_ref),
            )
            .chain(
                self.storage
                    .iter()
                    .map(|x| &x.constraint_expression.rhs_expression_ref),
            )
    }
}

impl Constraints<Building> {
    pub(crate) fn add_to_built_model(&self, model: &Rc<ModelData<Built>>) {
        for d in &self.storage {
            model.constraints.storage.push(d.build(model));
        }
    }
}
