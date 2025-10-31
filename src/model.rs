use crate::model_data::ModelData;
use crate::symbols::{Set, SetData, SetKind, SymbolData};

#[derive(Default)]
pub struct Model {
    data: ModelData,
}

impl Model {
    pub fn new() -> Self {
        Self::default()
    }

    // symbols

    pub fn set(&self) -> Set<'_> {
        let data = SetData {
            kind: SetKind::Independent,
        };

        let symbol_data = SymbolData {
            key: Default::default(),
            definition: Default::default(),
            data,
        };

        self.data.sets.push(self, symbol_data)
    }
}
