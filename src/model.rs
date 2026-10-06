use crate::pars::IntPar0Data;
use crate::scalars::{Int, IntData};
use crate::sets::{Bound, Range, RangeData};
use crate::symbol::Symbol;
use orx_imp_vec::ImpVec;

#[derive(Default)]
pub struct Model {
    // pars
    pub(crate) int_par0: ImpVec<IntPar0Data>,

    // int
    pub(crate) int: ImpVec<IntData>,

    // sets
    range: ImpVec<RangeData>,
}

impl Model {
    // pars
    pub fn int_par0(&self) -> Int<'_> {
        let data = IntPar0Data::new();
        let d = self.int_par0.imp_push_get_ref(data);

        let data = IntData::from((self, d));
        let d = self.int.imp_push_get_ref(data);

        Int(Symbol::new(self, d))
    }

    // sets
    pub fn range(&self, lower: impl Into<Bound>, upper: impl Into<Bound>) -> Range<'_> {
        let data = RangeData::new(lower.into(), upper.into());
        let d = self.range.imp_push_get_ref(data);
        Range::new(Symbol::new(self, d))
    }
}
