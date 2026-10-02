use crate::symbols::parameters::{Count, IntPar1, Par1};
use crate::symbols::sets::Set;
use crate::symbols::symbol_defn::SymbolDefinition;
use crate::symbols::{Model, Symbol};
use orx_imp_vec::ImpVec;

#[derive(Default, Debug)]
pub struct AllPars {
    count: ImpVec<SymbolDefinition>,
    par1: ImpVec<SymbolDefinition>,
    int_par1: ImpVec<SymbolDefinition>,
}

impl AllPars {
    pub fn count<'a>(&'a self, m: &'a Model) -> Count<'a> {
        let s = Symbol::new(m, self.count.imp_push_get_ref(Default::default()));
        Count::new(s)
    }

    pub fn par1<'a>(&'a self, m: &'a Model, i: Set<'a>) -> Par1<'a> {
        let s = Symbol::new(m, self.par1.imp_push_get_ref(Default::default()));
        Par1::new(s, i)
    }

    pub fn int_par1<'a>(&'a self, m: &'a Model, i: Set<'a>) -> IntPar1<'a> {
        let s = Symbol::new(m, self.int_par1.imp_push_get_ref(Default::default()));
        IntPar1::new(s, i)
    }
}
