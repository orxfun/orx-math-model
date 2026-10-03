use crate::symbols::parameters::Count;
use crate::symbols::sets::{IndexedSet, Set};
use crate::symbols::symbol_defn::SymbolDefinition;
use crate::symbols::{Model, Symbol};
use orx_imp_vec::ImpVec;

#[derive(Default, Debug)]
pub struct AllSets {
    sets: ImpVec<SymbolDefinition>,
    range_sets: ImpVec<SymbolDefinition>,
}

impl AllSets {
    pub fn set<'a>(&'a self, m: &'a Model) -> Set<'a> {
        let s = Symbol::new(m, self.sets.imp_push_get_ref(Default::default()));
        Set::new(s)
    }
}
