use crate::symbols::sets::{IndexSet, Set};
use crate::symbols::symbol_data::SymbolData;
use alloc::string::String;
use orx_imp_vec::ImpVec;

#[derive(Default, Debug)]
pub struct Model {
    name: String,
    sets: ImpVec<SymbolData>,
    index_sets: ImpVec<SymbolData>,
    range_sets: ImpVec<SymbolData>,
}

impl Model {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn set(&self, key: impl Into<String>) -> Set<'_> {
        let data = SymbolData::new(key.into(), None);
        let d = self.sets.imp_push_get_ref(data);
        Set::new(self, d)
    }

    pub fn indices(&self, key: impl Into<String>) -> IndexSet<'_> {
        let data = SymbolData::new(key.into(), None);
        let d = self.index_sets.imp_push_get_ref(data);
        IndexSet::new(self, d)
    }
}
