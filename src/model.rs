use crate::model_data::ModelData;
use crate::symbols::{DependentSetIndices, Set, SetCore, SetData, Symbol};

#[derive(Default)]
pub struct Model {
    pub(crate) data: ModelData,
}

impl Model {
    pub fn new() -> Self {
        Self::default()
    }

    // sets

    pub fn set(&self) -> Set<'_, 0> {
        let data = SetData::new(DependentSetIndices::new(core::iter::empty()));
        self.data.sets.push(self, Symbol::new(data)).set_ref()
    }

    pub(crate) fn dep_set<'m, const N: usize>(&'m self, sets: [Set<'m, 0>; N]) -> Set<'m, N> {
        let data = SetData::new(DependentSetIndices::new(sets.into_iter()));
        self.data.sets.push(self, Symbol::new(data)).set_ref()
    }

    // pub fn set_by_key(&self, key: &str) -> Option<Set<'_>> {
    //     self.data.sets.by_key(self, key).map(Set::from)
    // }

    // helpers

    pub(crate) fn set_at(&self, idx: usize) -> Option<SetCore<'_>> {
        self.data.sets.at(self, idx).map(SetCore::from)
    }
}
