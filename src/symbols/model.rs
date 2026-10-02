use crate::symbols::sets::{CatSet, NumSet};
use crate::symbols::symbol_data::SymbolData;
use alloc::string::String;
use orx_imp_vec::ImpVec;

pub struct Model {
    num_sets: ImpVec<SymbolData>,
    cat_sets: ImpVec<SymbolData>,
}

impl Model {
    pub fn num_set(&self, key: impl Into<String>) -> NumSet<'_> {
        let data = SymbolData::new(key.into(), None);
        let d = self.num_sets.imp_push_get_ref(data);
        NumSet::new(self, d)
    }

    pub fn cat_set(&self, key: impl Into<String>) -> CatSet<'_> {
        let data = SymbolData::new(key.into(), None);
        let d = self.cat_sets.imp_push_get_ref(data);
        CatSet::new(self, d)
    }
}
