use crate::symbols::{
    Element, ElementData, Idx, IdxData, Indices, IndicesData, Int, IntFun, Num, NumFun, Par,
    ParData, ParInt, ParIntData, PseudoInt, Set, SetFun,
};
use crate::symbols_d1::{
    ParD1DataDepr, ParD1Depr, ParIntD1Depr, ParIntD1DataDepr, SetOfIdxD1, SetOfIdxD1Data,
};
use orx_imp_vec::{Doubling, ImpVec, SplitVec};

type Pinned<T> = SplitVec<T, Doubling>;
type Bag<T> = ImpVec<T, Pinned<T>>;
fn new_bag<T>() -> Bag<T> {
    Bag::new()
}

pub struct Model<'d> {
    set: Bag<SetFun<'d>>,
    element: Bag<ElementData<'d>>,

    set_of_idx: Bag<IndicesData<'d>>,
    set_of_idx_d1: Bag<SetOfIdxD1Data<'d>>,
    idx: Bag<IdxData<'d>>,

    par: Bag<ParData>,
    par_d1: Bag<ParD1DataDepr<'d>>,
    par_int: Bag<ParIntData>,
    par_int_d1: Bag<ParIntD1DataDepr<'d>>,

    num: Bag<NumFun<'d>>,
    int: Bag<IntFun<'d>>,

    // symbols
    sym_set: Bag<Indices<'d, 'd>>,
}

impl<'d> Model<'d> {
    pub fn new() -> Self {
        Self {
            set: new_bag(),
            element: new_bag(),

            set_of_idx: new_bag(),
            set_of_idx_d1: new_bag(),
            idx: new_bag(),

            par: new_bag(),
            par_d1: new_bag(),
            par_int: new_bag(),
            par_int_d1: new_bag(),

            num: new_bag(),
            int: new_bag(),

            // symbols
            sym_set: new_bag(),
        }
    }

    // sets

    pub fn set(&self) -> Set<'_, 'd> {
        Set::new(self, self.set.push(SetFun::empty()))
    }

    pub fn indices(&self) -> Indices<'_, 'd> {
        self.add_set_of_idx(IndicesData::empty())
    }

    pub fn range<'m>(
        &'m self,
        begin: impl Into<PseudoInt<'m, 'd>>,
        exclusive_end: impl Into<PseudoInt<'m, 'd>>,
    ) -> Indices<'m, 'd> {
        let data = IndicesData::range(begin, exclusive_end);
        Indices::new(self, self.set_of_idx.push(data))
    }

    pub(crate) fn add_set_of_idx(&self, data: IndicesData<'d>) -> Indices<'_, 'd> {
        Indices::new(self, self.set_of_idx.push(data))
    }

    pub fn set_of_idx_d1<'m>(&'m self, set: Indices<'m, 'd>) -> SetOfIdxD1<'m, 'd> {
        let data = SetOfIdxD1Data::empty(set.fun_ptr());
        SetOfIdxD1::new(self, self.set_of_idx_d1.push(data))
    }

    // pars

    pub fn par(&self) -> Par<'_, 'd> {
        let data = ParData::new(0.0);
        Par::new(self, self.par.push(data))
    }

    pub fn par_int(&self) -> ParInt<'_, 'd> {
        let data = ParIntData::new(0);
        ParInt::new(self, self.par_int.push(data))
    }

    pub fn par_d1<'m>(&'m self, set: Indices<'m, 'd>) -> ParD1Depr<'m, 'd> {
        let data = ParD1DataDepr::empty(set.fun_ptr());
        ParD1Depr::new(self, self.par_d1.push(data))
    }

    pub fn par_int_d1<'m>(&'m self, set: Indices<'m, 'd>) -> ParIntD1Depr<'m, 'd> {
        let data = ParIntD1DataDepr::empty(set.fun_ptr());
        ParIntD1Depr::new(self, self.par_int_d1.push(data))
    }

    // idx

    pub(crate) fn element<'m>(&'m self, set: Set<'m, 'd>) -> Element<'m, 'd> {
        let data = ElementData::new(set.fun_ptr(), 0);
        Element::new(self, self.element.push(data))
    }

    pub(crate) fn idx<'m>(&'m self, set: Indices<'m, 'd>) -> Idx<'m, 'd> {
        let data = IdxData::new(set.fun_ptr(), 0);
        Idx::new(self, self.idx.push(data))
    }

    // scalars

    pub(crate) fn add_num<'m>(&'m self, fun: NumFun<'d>) -> Num<'m, 'd> {
        Num::new(self, self.num.push(fun))
    }

    pub(crate) fn add_int<'m>(&'m self, fun: IntFun<'d>) -> Int<'m, 'd> {
        Int::new(self, self.int.push(fun))
    }

    // symbols

    pub(crate) fn add_sym_set<'m>(&'m self, set: Indices<'d, 'd>) -> &'m Indices<'d, 'd> {
        let el = self.sym_set.push(set);
        unsafe { el.get() }
    }
}
