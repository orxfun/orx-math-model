use crate::pars::{IntPar0, IntPar0Data};
use crate::scalars::{Int, IntData};
// use crate::sets::{Bound, Range, RangeData};
use crate::symbol::Symbol;
use alloc::boxed::Box;
use orx_imp_vec::ImpVec;

#[derive(Default)]
pub struct Model {
    // pars
    pub(crate) int_par0: ImpVec<IntPar0Data>,

    // scalars
    pub(crate) int: ImpVec<IntData>,
}

impl Model {
    // pars
    pub fn int_par0(&self) -> IntPar0<'_> {
        let data = IntPar0Data::new();
        let d = self.int_par0.imp_push_get_ref(data);
        IntPar0(Symbol::new(self, d))
    }

    // scalars
    pub(crate) fn int(&self, value: Box<dyn Fn(&Model) -> i64>) -> Int<'_> {
        let data = IntData::new(value);
        let d = self.int.imp_push_get_ref(data);
        Int(Symbol::new(self, d))
    }
}
