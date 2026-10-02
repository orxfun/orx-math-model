use crate::symbols::sets::{IndexSet, Set};
use crate::symbols::symbol_data::SymbolData;
use crate::symbols::Model;
use alloc::string::String;
use orx_imp_vec::ImpVec;

#[derive(Default, Debug)]
pub struct AllSets {
    sets: ImpVec<SymbolData>,
    index_sets: ImpVec<SymbolData>,
    range_sets: ImpVec<SymbolData>,
}

impl AllSets {
    pub fn set<'a>(&'a self, m: &'a Model, key: impl Into<String>) -> Set<'a> {
        let data = SymbolData::new(key.into(), None);
        let d = self.sets.imp_push_get_ref(data);
        Set::new(m, d)
    }

    pub fn indices<'a>(&'a self, m: &'a Model, key: impl Into<String>) -> IndexSet<'a> {
        let data = SymbolData::new(key.into(), None);
        let d = self.index_sets.imp_push_get_ref(data);
        IndexSet::new(m, d)
    }
}
