use crate::symbols::{
    Element, ElementData, ElementStr, ElementStrData, Idx, IdxData, Indices, IndicesData, Int,
    IntFun, Num, NumFun, Par, ParData, ParInt, ParIntData, PseudoInt, Set, SetFun, SetStr,
    SetStrFun,
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

    set_str: Bag<SetStrFun<'d>>,
    element_str: Bag<ElementStrData<'d>>,

    set_of_idx: Bag<IndicesData<'d>>,
    idx: Bag<IdxData<'d>>,

    par: Bag<ParData>,
    par_int: Bag<ParIntData>,

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

            set_str: new_bag(),
            element_str: new_bag(),

            set_of_idx: new_bag(),
            idx: new_bag(),

            par: new_bag(),
            par_int: new_bag(),

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

    pub fn set_str(&self) -> SetStr<'_, 'd> {
        SetStr::new(self, self.set_str.push(SetStrFun::empty()))
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

    // pars

    pub fn par(&self) -> Par<'_, 'd> {
        let data = ParData::new(0.0);
        Par::new(self, self.par.push(data))
    }

    pub fn par_int(&self) -> ParInt<'_, 'd> {
        let data = ParIntData::new(0);
        ParInt::new(self, self.par_int.push(data))
    }

    // idx

    pub(crate) fn element<'m>(&'m self, set: Set<'m, 'd>) -> Element<'m, 'd> {
        let data = ElementData::new(set.fun_ptr(), 0);
        Element::new(self, self.element.push(data))
    }

    pub(crate) fn element_str<'m>(&'m self, set: SetStr<'m, 'd>) -> ElementStr<'m, 'd> {
        let data = ElementStrData::new(set.fun_ptr(), "");
        ElementStr::new(self, self.element_str.push(data))
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
