use crate::model_data::ModelData;
use crate::no_std_types::ColSet;
use crate::symbols::{Set, SetData, SymbolData};

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
        let data = SetData::new([]);
        self.data.sets.push(self, SymbolData::new(data))
    }

    pub(crate) fn dep_set<'m, const N: usize>(&'m self, sets: [Set<'m>; N]) -> Set<'m> {
        let sets = sets.to_vec();
        let mut indices = ColSet::<usize>::new();
        for set in &sets {
            indices.extend(set.symbol().data.data.depends_on_indices());
            indices.insert(
                self.data
                    .sets
                    .index_of(set.symbol())
                    .expect("the set does not exists in the model!"),
            );
        }

        let data = SetData::new(indices);
        self.data.sets.push(self, SymbolData::new(data))
    }

    // helpers

    pub(crate) fn set_at(&self, idx: usize) -> Option<Set<'_>> {
        self.data.sets.at(self, idx).map(Set::from)
    }
}
