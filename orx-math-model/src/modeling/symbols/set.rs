use super::data::set::set_elements_debug::SetElementsAndCore;
use super::data::set_data::SetData;
use crate::modeling::reference::{HasRef, SymRef};
use crate::modeling::stages::ModelStages;
use crate::modeling::symbol_collections::symbol_collection::SymbolCollection;
use std::fmt::{Debug, Display};

#[derive(Clone, Copy)]
pub struct Set<B: ModelStages>(SymRef<B>);
impl<B: ModelStages> HasRef<B> for Set<B> {
    fn new(index: SymRef<B>) -> Self {
        Self(index)
    }
    fn sym_ref(&self) -> &SymRef<B> {
        &self.0
    }
}

impl<B: ModelStages> Display for Set<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.data().key)
    }
}

impl<B: ModelStages> Debug for Set<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.data();
        let elements_and_core = SetElementsAndCore::new(self.core(), &data.elements);
        f.debug_struct("Set")
            .field("key", &data.key)
            .field("definition", &data.definition)
            .field("elements", &elements_and_core)
            .finish()
    }
}

// data
impl<B: ModelStages> Set<B> {
    pub(crate) fn data(&self) -> &SetData<B> {
        self.core().sets.get(self.sym_idx())
    }
}
