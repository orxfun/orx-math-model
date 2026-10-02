use crate::symbols::parameters::{Count, Par1};
use crate::symbols::sets::Set;
use crate::symbols::symbol_data::SymbolData;
use crate::symbols::Model;
use orx_imp_vec::ImpVec;

#[derive(Default, Debug)]
pub struct AllPars {
    counts: ImpVec<SymbolData>,
}

impl AllPars {
    pub fn count<'a>(&'a self, m: &'a Model) -> Count<'a> {
        Count::new(m, self.counts.imp_push_get_ref(Default::default()))
    }

    pub fn par1<'a>(&'a self, m: &'a Model, i: Set<'a>) -> Par1<'a> {
        Par1::new(m, self.counts.imp_push_get_ref(Default::default()), i)
    }
}
