use crate::symbols::parameters::AllPars;
use crate::symbols::sets::{AllSets, CatSet};
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Default, Debug)]
pub struct Model {
    name: String,
    sets: AllSets,
    pars: AllPars,

    // build
    elements: Vec<usize>,
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
        self.sets.add_cat(self)
    }

    // pars
}

// internal

impl Model {}
