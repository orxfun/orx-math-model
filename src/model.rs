use crate::pars::IntPar0Data;
use crate::pos::Pos;
use crate::scalar_int::{FunIntData, Int, IntData, RangeElemData};
use crate::sets::{Bound, Range, RangeData, Set, SetData};
use crate::symbol::Symbol;
use alloc::boxed::Box;
use orx_imp_vec::{ImpVec, PinnedVec};

#[derive(Default)]
pub struct Model {
    // pars
    pub(crate) int_par0: ImpVec<IntPar0Data>,

    // int
    pub(crate) int: ImpVec<IntData>,
    pub(crate) fun_int: ImpVec<FunIntData>,

    // sets
    pub(crate) range: ImpVec<RangeData>,
    pub(crate) set: ImpVec<SetData>,

    // elements
    pub(crate) range_elem: ImpVec<RangeElemData>,

    // resolve
    pub(crate) range_elem_value: ImpVec<i64>,
}

impl Model {
    // pars
    pub fn int_par0(&self) -> Int<'_> {
        let data = IntPar0Data::new();
        let d = self.int_par0.imp_push_get_ref(data);
        self.int(IntData::from((self, d)))
    }

    // int
    fn int(&self, data: IntData) -> Int<'_> {
        let d = self.int.imp_push_get_ref(data);
        Int(Symbol::new(self, d))
    }

    pub(crate) fn fun_int(&self, fun: Box<dyn Fn(&Model) -> i64>) -> Int<'_> {
        let data = FunIntData::new(fun);
        let d = self.fun_int.imp_push_get_ref(data);
        self.int(IntData::from((self, d)))
    }

    // sets
    pub fn range(&self, lower: impl Into<Bound>, upper: impl Into<Bound>) -> Range<'_> {
        let data = RangeData::new(lower.into(), upper.into());
        let d = self.range.imp_push_get_ref(data);
        Range::new(Symbol::new(self, d))
    }

    pub fn set(&self) -> Set<'_> {
        let data = SetData;
        let d = self.set.imp_push_get_ref(data);
        Set::new(Symbol::new(self, d))
    }

    // elements
    pub(crate) fn range_elem(&self, range: Range<'_>) -> Int<'_> {
        let range_pos = range.0.pos();
        let value_pos = Pos::new_for(&self.range_elem);
        let data = RangeElemData::new(range_pos, value_pos);
        let d = self.range_elem.imp_push_get_ref(data);
        self.range_elem_value.imp_push(0);
        self.int(IntData::from(d))
    }
}
