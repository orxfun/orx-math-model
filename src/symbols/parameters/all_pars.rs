use crate::symbols::parameters::IntPar0;
use crate::symbols::symbol_defn::SymbolDefinition;
use crate::symbols::{Model, Symbol};
use orx_imp_vec::ImpVec;

#[derive(Default, Debug)]
pub struct Pars {
    int_par0: ImpVec<SymbolDefinition>,

    // run data
    int_par0_values: ImpVec<i64>,
}

impl Pars {
    pub fn add_par0<'a>(&'a self, m: &'a Model) -> IntPar0<'a> {
        self.int_par0_values.imp_push(0);
        let s = Symbol::new(m, self.int_par0.imp_push_get_ref(Default::default()));
        IntPar0::new(s)
    }
}
