use crate::symbols::parameters::{AllPars, Count};
use crate::symbols::sets::{AllSets, IndexSet, Set};
use alloc::string::String;

#[derive(Default, Debug)]
pub struct Model {
    name: String,
    sets: AllSets,
    pars: AllPars,
}

impl Model {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    // sets

    pub fn set(&self, key: impl Into<String>) -> Set<'_> {
        self.sets.set(self, key)
    }

    pub fn indices(&self, key: impl Into<String>) -> IndexSet<'_> {
        self.sets.indices(self, key)
    }

    // pars

    pub fn count(&self, key: impl Into<String>) -> Count<'_> {
        self.pars.count(self, key)
    }
}
