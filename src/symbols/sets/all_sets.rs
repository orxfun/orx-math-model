use crate::symbols::parameters::Count;
use crate::symbols::sets::{IndexedSet, RangeSet, CatSet};
use crate::symbols::symbol_defn::SymbolDefinition;
use crate::symbols::{Model, Symbol};
use orx_imp_vec::{ImpVec, PinnedVec, SplitVec};

#[derive(Default, Debug)]
pub struct AllSets {
    cat: ImpVec<SymbolDefinition>,
    range: ImpVec<SymbolDefinition>,
}

impl AllSets {
    pub fn set<'a>(&'a self, m: &'a Model) -> CatSet<'a> {
        let s = Symbol::new(m, self.cat.imp_push_get_ref(Default::default()));
        CatSet::new(s)
    }

    pub fn pos_of_cat(&self, set: CatSet<'_>) -> Option<usize> {
        self.cat.index_of(&set.symbol().d)
    }

    pub fn pos_of_range(&self, set: RangeSet<'_>) -> Option<usize> {
        self.range.index_of(&set.symbol().d)
    }
}
