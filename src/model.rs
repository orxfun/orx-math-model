use crate::model_data::ModelData;
use crate::symbols::{DependentSetIndices, Elements, Set, SetCore, SetData, Symbol};

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
        let dep = DependentSetIndices::new(core::iter::empty());
        let elem = Elements::empty(0);
        let data = SetData::new(dep, elem);
        self.data.sets.push(self, Symbol::new(data)).set_ref()
    }

    pub(crate) fn dep_set<'m, const N: usize>(&'m self, sets: [Set<'m, 0>; N]) -> Set<'m, N> {
        let dep = DependentSetIndices::new(sets.into_iter());
        let elem = Elements::empty(N);
        let data = SetData::new(dep, elem);
        self.data.sets.push(self, Symbol::new(data)).set_ref()
    }

    pub fn set_by_key<const N: usize>(&self, key: &str) -> Option<Set<'_, N>> {
        let core = self.data.sets.by_key(self, key).map(SetCore::from);
        core.and_then(|x| x.set_ref_checked::<N>())
    }

    // helpers

    pub(crate) fn set_at(&self, idx: usize) -> Option<SetCore<'_>> {
        self.data.sets.at(self, idx).map(SetCore::from)
    }
}
