use crate::symbols::sets::{CatSet, RangeSet};
use crate::symbols::symbol_defn::SymbolDefinition;
use crate::symbols::{Model, Symbol};
use orx_imp_vec::ImpVec;

#[derive(Default, Debug)]
pub struct AllSets {
    cat: ImpVec<SymbolDefinition>,
    range: ImpVec<SymbolDefinition>,

    cat_run_idx: ImpVec<usize>,
    range_run_idx: ImpVec<usize>,
}

impl AllSets {
    pub fn add_cat<'a>(&'a self, m: &'a Model) -> CatSet<'a> {
        self.cat_run_idx.imp_push(0);
        let s = Symbol::new(m, self.cat.imp_push_get_ref(Default::default()));
        CatSet::new(s)
    }

    pub fn add_range<'a>(&'a self, m: &'a Model) -> RangeSet<'a> {
        self.range_run_idx.imp_push(0);
        let s = Symbol::new(m, self.cat.imp_push_get_ref(Default::default()));
        RangeSet::new(s)
    }
}
