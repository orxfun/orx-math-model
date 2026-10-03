use crate::symbols::parameters::Pars;
use crate::symbols::sets::{CatSet, Sets};
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Default)]
pub struct Model {
    name: String,
    sets: Sets,
    pars: Pars,

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
