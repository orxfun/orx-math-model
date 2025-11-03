use crate::model_data::ModelData;
use crate::symbols::{Set, SetData, Symbol};

#[derive(Default)]
pub struct Model {
    pub(crate) data: ModelData,
}

impl Model {
    pub fn new() -> Self {
        Self::default()
    }

    // sets

    pub fn set(&self) -> Set<'_> {
        let data = SetData::new();
        let set_core = self.data.sets.push(self, Symbol::new(data));
        todo!()
    }

    pub(crate) fn dep_set<'m, const N: usize>(&'m self, sets: [Set<'m>; N]) -> Set<'m> {
        let sets = sets.to_vec();
        let mut data = SetData::new();
        for set in &sets {
            // data.add_depending_set(*set);
            todo!()
        }
        let set_core = self.data.sets.push(self, Symbol::new(data));
        todo!()
    }

    pub fn set_by_key(&self, key: &str) -> Option<Set<'_>> {
        self.data.sets.by_key(self, key).map(Set::from)
    }

    // helpers

    pub(crate) fn set_at(&self, idx: usize) -> Option<Set<'_>> {
        self.data.sets.at(self, idx).map(Set::from)
    }
}
