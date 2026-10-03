use crate::symbols::parameters::{IntPar0, IntPar0Data};
use crate::symbols::scalars::{Int, IntData};
use crate::symbols::{Model, Symbol, SymbolDefinition};
use alloc::boxed::Box;
use orx_imp_vec::{ImpVec, PinnedVec};

#[derive(Default)]
pub struct Scalars {
    pub int: ImpVec<IntData>,
}

impl Scalars {
    pub fn add_int<'a>(
        &'a self,
        m: &'a Model,
        def: SymbolDefinition,
        create: Box<dyn Fn(&Model) -> i64>,
    ) -> Int<'a> {
        let data = IntData::new(def, create);
        let d = self.int.imp_push_get_ref(data);
        Int::new(m, d)
    }
}
