use super::smallvec::Smallvec;
use crate::{
    modeling::stages::{Building, Built, ModelStages},
    modeling::{
        model_data::ModelData,
        reference::{HasRef, SymRef},
        symbols::scalar::Scalar,
    },
};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub(crate) struct VarScalarRefs<B: ModelStages>(Smallvec<SymRef<B>>);

impl<B: ModelStages> VarScalarRefs<B> {
    pub(crate) fn from_scalars<const D: usize>(scalars: &[Scalar<B>; D]) -> Self {
        let refs_vec = scalars.clone().map(|s| s.sym_ref().clone()).into();
        Self(refs_vec)
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
    pub(crate) fn as_slice(&self) -> &[SymRef<B>] {
        &self.0
    }
}

impl<const D: usize, B: ModelStages> From<[SymRef<B>; D]> for VarScalarRefs<B> {
    fn from(array: [SymRef<B>; D]) -> Self {
        VarScalarRefs(array.into())
    }
}

impl VarScalarRefs<Building> {
    pub(crate) fn build(&self, model: &Rc<ModelData<Built>>) -> VarScalarRefs<Built> {
        VarScalarRefs(self.0.build(model))
    }
}
