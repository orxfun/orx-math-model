use crate::symbols::parameters::{AllPars, Count, IntPar1, Par1};
use crate::symbols::sets::{AllSets, CatSet, IndexedSet};
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

    pub fn cat_set(&self) -> CatSet<'_> {
        self.sets.set(self)
    }

    // pars

    pub fn count(&self) -> Count<'_> {
        self.pars.count(self)
    }

    pub fn par1<'a>(&'a self, i: CatSet<'a>) -> Par1<'a> {
        self.pars.par1(self, i)
    }

    pub fn int_par1<'a>(&'a self, i: CatSet<'a>) -> IntPar1<'a> {
        self.pars.int_par1(self, i)
    }
}

// internal

impl Model {}
