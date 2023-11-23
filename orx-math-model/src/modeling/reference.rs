use super::{model_data::ModelData, model_data_ref::ModelDataRef};
use crate::modeling::stages::{Building, Built, ModelStages};
use std::{fmt::Debug, rc::Rc};

#[derive(Clone, Copy)]
pub(crate) struct SymRef<B: ModelStages> {
    pub(super) core_ref: ModelDataRef<B>,
    pub(crate) sym_idx: usize,
}

impl SymRef<Building> {
    pub(crate) fn new(core: &Rc<ModelData<Building>>, sym_idx: usize) -> Self {
        let ptr_core = Rc::as_ptr(core);
        let model_reference = Building(ptr_core);
        let core_ref = ModelDataRef { model_reference };
        Self { core_ref, sym_idx }
    }

    pub(crate) fn build(&self, model: Rc<ModelData<Built>>) -> SymRef<Built> {
        let model_reference = Built(model.clone());
        let core_ref = ModelDataRef { model_reference };
        let sym_idx = self.sym_idx;
        SymRef { core_ref, sym_idx }
    }
}

impl<B: ModelStages> SymRef<B> {
    pub(crate) fn core(&self) -> &ModelData<B> {
        self.core_ref.core()
    }
}
impl<B: ModelStages> Debug for SymRef<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SymRef")
            .field("sym_idx", &self.sym_idx)
            .finish()
    }
}

pub(crate) trait HasRef<B: ModelStages> {
    fn new(index: SymRef<B>) -> Self;
    fn sym_ref(&self) -> &SymRef<B>;
    fn core(&self) -> &ModelData<B> {
        self.sym_ref().core()
    }
    fn sym_idx(&self) -> usize {
        self.sym_ref().sym_idx
    }
}
