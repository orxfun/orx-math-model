use crate::symbols::parameters::Count;
use crate::symbols::symbol_data::SymbolData;
use crate::symbols::Model;
use alloc::string::String;
use orx_imp_vec::ImpVec;

#[derive(Default, Debug)]
pub struct AllPars {
    counts: ImpVec<SymbolData>,
}

impl AllPars {
    pub fn count<'a>(&'a self, m: &'a Model, key: impl Into<String>) -> Count<'a> {
        let data = SymbolData::new(key.into(), None);
        let d = self.counts.imp_push_get_ref(data);
        Count::new(m, d)
    }
}
