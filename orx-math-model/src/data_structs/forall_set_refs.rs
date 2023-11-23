use super::smallvec::Smallvec;
use crate::{
    modeling::stages::{Building, Built, ModelStages},
    modeling::{model_data::ModelData, reference::SymRef},
};
use std::{ops::Deref, rc::Rc};

#[derive(Debug, Clone)]
pub(crate) struct ForAllSetRefs<B: ModelStages>(Smallvec<SymRef<B>>);
impl<B: ModelStages> ForAllSetRefs<B> {
    pub(crate) fn new(refs_vec: Smallvec<SymRef<B>>) -> Self {
        Self(refs_vec)
    }
}

impl<B: ModelStages> ForAllSetRefs<B> {
    pub(crate) fn empty() -> Self {
        Self(Smallvec::empty())
    }
}

impl<B: ModelStages> Deref for ForAllSetRefs<B> {
    type Target = [SymRef<B>];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ForAllSetRefs<Building> {
    pub(crate) fn build(&self, model: &Rc<ModelData<Built>>) -> ForAllSetRefs<Built> {
        ForAllSetRefs(self.0.build(model))
    }
}
