use crate::symbols::parameters::{IntPar0, Pars};
use crate::symbols::scalars::Scalars;
use crate::symbols::sets::{CatSet, Sets};
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Default)]
pub struct Model {
    pub(crate) name: String,

    pub(crate) sets: Sets,
    pub(crate) pars: Pars,
    pub(crate) scalars: Scalars,
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

    pub fn int_par0(&self) -> IntPar0<'_> {
        todo!()
    }
}

// internal

impl Model {}
