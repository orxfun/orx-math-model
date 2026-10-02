use crate::symbols::parameters::Count;
use crate::symbols::sets::{IndexedSet, Set};
use crate::symbols::symbol_defn::SymbolDefinition;
use crate::symbols::Model;
use orx_imp_vec::ImpVec;

#[derive(Default, Debug)]
pub struct AllSets {
    sets: ImpVec<SymbolDefinition>,
    index_sets: ImpVec<SymbolDefinition>,
    range_sets: ImpVec<SymbolDefinition>,
}

impl AllSets {
    pub fn set<'a>(&'a self, m: &'a Model) -> Set<'a> {
        Set::new(m, self.sets.imp_push_get_ref(Default::default()))
    }

    pub fn indexed_set<'a>(&'a self, m: &'a Model, count: Count<'a>) -> IndexedSet<'a> {
        let d = self.index_sets.imp_push_get_ref(Default::default());
        IndexedSet::new(m, d, count)
    }
}
