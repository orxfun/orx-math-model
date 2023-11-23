use super::smallvec::Smallvec;
use crate::{
    modeling::stages::{Building, Built, ModelStages},
    modeling::{model_data::ModelData, reference::SymRef},
};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub(crate) struct SumOverSetRefs<B: ModelStages>(Smallvec<SymRef<B>>);

impl<B: ModelStages> SumOverSetRefs<B> {
    pub(crate) fn iter(&self) -> impl Iterator<Item = &SymRef<B>> {
        self.0.iter()
    }
}

impl<const D: usize, B: ModelStages> From<[SymRef<B>; D]> for SumOverSetRefs<B> {
    fn from(value: [SymRef<B>; D]) -> Self {
        Self(value.into())
    }
}

impl SumOverSetRefs<Building> {
    pub(crate) fn build(&self, model: &Rc<ModelData<Built>>) -> SumOverSetRefs<Built> {
        SumOverSetRefs(self.0.build(model))
    }
}
