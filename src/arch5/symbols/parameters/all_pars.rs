use crate::symbols::parameters::{IntPar0, IntPar0Data};
use crate::symbols::{Model, Symbol};
use orx_imp_vec::{ImpVec, PinnedVec};

#[derive(Default)]
pub struct Pars {
    pub int_par0: ImpVec<IntPar0Data>,
}

impl Pars {
    pub fn add_par0<'a>(&'a self, m: &'a Model) -> IntPar0<'a> {
        let pos = self.int_par0.len();
        let data = IntPar0Data::new(Default::default(), 0);
        let d = self.int_par0.imp_push_get_ref(data);
        IntPar0::new(m, d, pos)
    }
}
