use crate::{
    model_data::ModelData,
    symbols::{Set, SetData, SymbolData},
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
        // let data = SetData {};
        // let symbol_data = SymbolData {
        //     key: key.into(),
        //     definition: Default::default(),
        //     data,
        // };

        // self.data.sets.push(self, symbol_data)

        todo!()
    }
}
