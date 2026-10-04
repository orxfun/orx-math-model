use crate::{
    pars::{IntPar0, IntPar0Data},
    symbol::Symbol,
};
use orx_imp_vec::ImpVec;

#[derive(Default)]
pub struct Model {
    // pars
    pub(crate) int_par0: ImpVec<IntPar0Data>,
}

impl Model {
    // pars
    pub fn int_par0(&self) -> IntPar0<'_> {
        let data = IntPar0Data::new(0);
        let d = self.int_par0.imp_push_get_ref(data);
        IntPar0(Symbol::new(self, d))
    }
}
