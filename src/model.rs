use crate::model_data::ModelData;
use crate::symbols::{IntoSet, Set, SetData, SetKind, SymbolData};
use alloc::boxed::Box;

#[derive(Default)]
pub struct Model {
    data: ModelData,
}

impl Model {
    pub fn new() -> Self {
        Self::default()
    }

    // symbols

    pub fn set(&self, data: impl IntoSet) -> Set<'_> {
        let set = data.into_set();
        let data = Box::new(set);
        let data = SetData {
            kind: SetKind::Independent(data),
        };

        let symbol_data = SymbolData {
            key: Default::default(),
            definition: Default::default(),
            data,
        };

        self.data.sets.push(self, symbol_data)
    }
}
