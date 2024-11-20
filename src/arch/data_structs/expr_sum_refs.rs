use super::smallvec::Smallvec;
use crate::modeling::stages::{Building, Built, ModelStages};
use crate::modeling::symbol_collections::symbol_collection::SymbolCollection;
use crate::modeling::{model_data::ModelData, reference::SymRef, symbols::sum::Sum};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub(crate) struct ExprSumRefs<B: ModelStages>(Smallvec<SymRef<B>>);

impl<B: ModelStages> ExprSumRefs<B> {
    pub(crate) fn empty() -> Self {
        Self(Smallvec::empty())
    }
    pub(crate) fn iter(&self) -> impl Iterator<Item = &SymRef<B>> {
        self.0.iter()
    }
}

impl<B: ModelStages, I: Iterator<Item = SymRef<B>>> From<I> for ExprSumRefs<B> {
    fn from(values: I) -> Self {
        let mut indices = ExprSumRefs::empty();
        for value in values {
            indices.0.push(value);
        }
        indices
    }
}
impl<B: ModelStages> Deref for ExprSumRefs<B> {
    type Target = [SymRef<B>];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ExprSumRefs<Building> {
    pub(crate) fn build(&self, model: &Rc<ModelData<Built>>) -> ExprSumRefs<Built> {
        ExprSumRefs(self.0.build(model))
    }
}

pub(crate) struct ExprSums<B: ModelStages>(Smallvec<Sum<B>>);
impl<B: ModelStages> ExprSums<B> {
    pub(crate) fn empty() -> Self {
        Self(Smallvec::empty())
    }
    pub(crate) fn singleton(sum: Sum<B>) -> Self {
        Self(Smallvec::singleton(sum))
    }

    pub(crate) fn from_refs(core: &ModelData<B>, refs: &ExprSumRefs<B>) -> ExprSums<B> {
        refs.iter()
            .map(|x| core.sums.get(x.sym_idx).symbol.clone())
            .into()
    }
    pub(crate) fn iter(&self) -> impl Iterator<Item = Sum<B>> + '_ {
        self.0.iter().cloned()
    }
}
impl<B: ModelStages, I: Iterator<Item = Sum<B>>> From<I> for ExprSums<B> {
    fn from(values: I) -> Self {
        let mut indices = ExprSums::empty();
        for value in values {
            indices.0.push(value);
        }
        indices
    }
}
