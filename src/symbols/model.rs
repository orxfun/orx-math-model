use crate::symbols::parameters::{AllPars, Count, Par1};
use crate::symbols::sets::{AllSets, IndexedSet, Set};
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

    pub fn set(&self) -> Set<'_> {
        self.sets.set(self)
    }

    pub fn indexed_set<'a>(&'a self, count: Count<'a>) -> IndexedSet<'a> {
        self.sets.indexed_set(self, count)
    }

    // pars

    pub fn count(&self) -> Count<'_> {
        self.pars.count(self)
    }

    pub fn par1<'a>(&'a self, i: Set<'a>) -> Par1<'a> {
        self.pars.par1(self, i)
    }
}
