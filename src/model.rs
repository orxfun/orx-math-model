use crate::{
    model_data::ModelData,
    symbols::{Set, SetData},
};
use alloc::string::String;

#[derive(Default)]
pub struct Model {
    data: ModelData,
}

impl Model {
    pub fn new() -> Self {
        Self::default()
    }

    // symbols

    pub fn set(&self, key: impl Into<String>) -> Set<'_> {
        self.data.sets.push(self, SetData::new(key.into()))
    }
}
