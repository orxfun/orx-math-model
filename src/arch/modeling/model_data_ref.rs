use super::{model_data::ModelData, reference::SymRef};
use crate::modeling::{
    stages::ModelStages, symbol_collections::symbol_collection::SymbolCollection,
};

#[derive(Clone, Copy)]
pub struct ModelDataRef<B: ModelStages> {
    pub(super) model_reference: B,
}

impl<B: ModelStages> ModelDataRef<B> {
    pub(crate) fn core(&self) -> &ModelData<B> {
        self.model_reference.core()
    }
    pub(crate) fn is_same_model(&self, other: &Self) -> bool {
        self.model_reference.is_same_model(&other.model_reference)
    }
    fn new_symref_with_idx(&self, sym_idx: usize) -> SymRef<B> {
        SymRef {
            core_ref: self.clone(),
            sym_idx,
        }
    }

    pub(crate) fn new_symref_scalar(&self) -> SymRef<B> {
        self.new_symref_with_idx(self.core().scalars.len())
    }
    pub(crate) fn new_symref_var(&self) -> SymRef<B> {
        self.new_symref_with_idx(self.core().vars.len())
    }
    pub(crate) fn new_symref_term(&self) -> SymRef<B> {
        self.new_symref_with_idx(self.core().terms.len())
    }
    pub(crate) fn new_symref_expression(&self) -> SymRef<B> {
        self.new_symref_with_idx(self.core().expressions.len())
    }
    pub(crate) fn new_symref_sum(&self) -> SymRef<B> {
        self.new_symref_with_idx(self.core().sums.len())
    }
    pub(crate) fn new_symref_constraint(&self) -> SymRef<B> {
        self.new_symref_with_idx(self.core().constraints.len())
    }
    pub(crate) fn new_symref_objective(&self) -> SymRef<B> {
        self.new_symref_with_idx(self.core().objectives.len())
    }
}
