use crate::pars::{IntPar0, IntPar0Data};
use crate::scalars::{Int, IntData};
use crate::sets::{Bound, Range, RangeData};
use crate::symbol::Symbol;
use alloc::boxed::Box;
use orx_imp_vec::ImpVec;

#[derive(Default)]
pub struct Model<'a> {
    // pars
    int_par0: ImpVec<IntPar0Data>,

    // scalars
    int: ImpVec<IntData>,

    // sets
    range: ImpVec<RangeData>,
}

impl<'a> Model<'a> {
    // helper
    fn symbol<D>(&self, data: &D) -> Symbol<'a, D> {
        let m = unsafe { &*(self as *const Model) };
        let d = unsafe { &*(data as *const D) };
        Symbol::new(m, d)
    }

    // pars
    pub fn int_par0<'b>(&'a self) -> IntPar0<'b>
    where
        'a: 'b,
    {
        let data = IntPar0Data::new(0);
        let d: &IntPar0Data = self.int_par0.imp_push_get_ref(data);
        // IntPar0(self.symbol(d))
        todo!()
    }

    // scalars
    pub(crate) fn int(&'a self, value: Box<dyn Fn() -> i64 + 'a>) -> Int<'a> {
        let data = IntData::new(value);
        let d = self.int.imp_push_get_ref(data);
        Int(Symbol::new(self, d))
    }

    // sets
    pub fn range(&self, lower: impl Into<Bound<'a>>, upper: impl Into<Bound<'a>>) -> Range<'a> {
        let data = RangeData::new(lower.into(), upper.into());
        let d = self.range.imp_push_get_ref(data);
        Range(self.symbol(d))
    }
}
