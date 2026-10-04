use crate::pars::{IntPar0, IntPar0Data};
use crate::scalars::{Int, IntData};
use crate::symbol::Symbol;
use alloc::boxed::Box;
use orx_imp_vec::ImpVec;

#[derive(Default)]
pub struct Model<'a> {
    // pars
    pub(crate) int_par0: ImpVec<IntPar0Data>,

    // scalars
    pub(crate) int: ImpVec<IntData<'a>>,
}

impl<'a> Model<'a> {
    // pars
    pub fn int_par0(&'a self) -> IntPar0<'a> {
        let data = IntPar0Data::new(0);
        let d = self.int_par0.imp_push_get_ref(data);
        IntPar0(Symbol::new(self, d))
    }

    // scalars
    pub(crate) fn int(&'a self, value: Box<dyn Fn() -> i64 + 'a>) -> Int<'a> {
        let data = IntData::new(value);
        let d = self.int.imp_push_get_ref(data);
        Int(Symbol::new(self, d))
    }
}
