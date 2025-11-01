use crate::model_data::ModelData;
use crate::stages::Stage;
use crate::symbols::{Set, SetData, Symbol};
use crate::Modeling;

#[derive(Default)]
pub struct Model<S = Modeling>
where
    S: Stage,
{
    pub(crate) data: ModelData<S>,
}

impl Model<Modeling> {
    pub fn new() -> Self {
        Self::default()
    }

    // sets

    pub fn set(&self) -> Set<'_> {
        let data = SetData::new();
        self.data.sets.push(self, Symbol::new(data))
    }
}

impl<S: Stage> Model<S> {
    // sets

    pub fn set_by_key(&self, key: &str) -> Option<Set<'_>> {
        self.data.sets.by_key(self, key).map(Set::from)
    }

    pub(crate) fn dep_set<'m, const N: usize>(&'m self, sets: [Set<'m>; N]) -> Set<'m> {
        let sets = sets.to_vec();
        let mut data = SetData::new();
        for set in &sets {
            data.add_depending_set(*set);
        }
        self.data.sets.push(self, Symbol::new(data))
    }

    // helpers

    pub(crate) fn set_at(&self, idx: usize) -> Option<Set<'_>> {
        self.data.sets.at(self, idx).map(Set::from)
    }
}
