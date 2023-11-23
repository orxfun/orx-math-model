use super::smallvec::Smallvec;
use crate::modeling::stages::{Building, Built};
use crate::modeling::symbol_collections::symbol_collection::SymbolCollection;
use crate::{
    modeling::stages::ModelStages,
    modeling::{
        model_data::ModelData,
        reference::{HasRef, SymRef},
        symbols::term::Term,
    },
};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub(crate) struct ExprTermRefs<B: ModelStages>(Smallvec<SymRef<B>>);

impl<B: ModelStages> ExprTermRefs<B> {
    pub(crate) fn empty() -> Self {
        Self(Smallvec::empty())
    }
    pub(crate) fn iter(&self) -> impl Iterator<Item = &SymRef<B>> {
        self.0.iter()
    }
}

impl<B: ModelStages, I: Iterator<Item = SymRef<B>>> From<I> for ExprTermRefs<B> {
    fn from(values: I) -> Self {
        let mut indices = ExprTermRefs::empty();
        for value in values {
            indices.0.push(value);
        }
        indices
    }
}
impl<B: ModelStages> From<ExprTerms<B>> for ExprTermRefs<B> {
    fn from(value: ExprTerms<B>) -> Self {
        value.0.iter().map(|x| x.sym_ref().clone()).into()
    }
}
impl<B: ModelStages> Deref for ExprTermRefs<B> {
    type Target = [SymRef<B>];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ExprTermRefs<Building> {
    pub(crate) fn build(&self, model: &Rc<ModelData<Built>>) -> ExprTermRefs<Built> {
        ExprTermRefs(self.0.build(model))
    }
}

pub(crate) struct ExprTerms<B: ModelStages>(Smallvec<Term<B>>);
impl<B: ModelStages> ExprTerms<B> {
    pub(crate) fn empty() -> Self {
        Self(Smallvec::empty())
    }
    pub(crate) fn singleton(term: Term<B>) -> Self {
        Self(Smallvec::singleton(term))
    }

    pub(crate) fn from_indices(core: &ModelData<B>, indices: &ExprTermRefs<B>) -> ExprTerms<B> {
        indices
            .iter()
            .map(|x| core.terms.get(x.sym_idx).symbol.clone())
            .into()
    }
    pub(crate) fn iter(&self) -> impl Iterator<Item = Term<B>> + '_ {
        self.0.iter().cloned()
    }
}
impl<B: ModelStages, I: Iterator<Item = Term<B>>> From<I> for ExprTerms<B> {
    fn from(values: I) -> Self {
        let mut indices = ExprTerms::empty();
        for value in values {
            indices.0.push(value);
        }
        indices
    }
}
