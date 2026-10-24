use crate::symbols::{
    IdxDataDepr, IdxDepr, Int, IntFun, Num, NumFun, Par, ParData, ParInt, ParIntData, PseudoInt,
    Set, SetFun, SetOfIdx, SetOfIdxData,
};
use crate::symbols_d1::{ParD1, ParD1Data, ParIntD1, ParIntD1Data, SetOfIdxD1, SetOfIdxD1Data};
use orx_imp_vec::{Doubling, ImpVec, SplitVec};

type Pinned<T> = SplitVec<T, Doubling>;
type Bag<T> = ImpVec<T, Pinned<T>>;
fn new_bag<T>() -> Bag<T> {
    Bag::new()
}

pub struct Model<'d> {
    set: Bag<SetFun<'d>>,

    set_of_idx: Bag<SetOfIdxData<'d>>,
    set_of_idx_d1: Bag<SetOfIdxD1Data<'d>>,

    par: Bag<ParData>,
    par_d1: Bag<ParD1Data<'d>>,
    par_int: Bag<ParIntData>,
    par_int_d1: Bag<ParIntD1Data<'d>>,

    idx: Bag<IdxDataDepr<'d>>,

    num: Bag<NumFun<'d>>,
    int: Bag<IntFun<'d>>,
}

impl<'d> Model<'d> {
    pub fn new() -> Self {
        Self {
            set: new_bag(),

            set_of_idx: new_bag(),
            set_of_idx_d1: new_bag(),

            par: new_bag(),
            par_d1: new_bag(),
            par_int: new_bag(),
            par_int_d1: new_bag(),

            idx: new_bag(),

            num: new_bag(),
            int: new_bag(),
        }
    }

    // sets

    // pub fn set(&self) -> Set

    pub fn set_of_idx(&self) -> SetOfIdx<'_, 'd> {
        self.add_set_of_idx(SetOfIdxData::empty())
    }

    pub fn range<'m>(
        &'m self,
        begin: impl Into<PseudoInt<'m, 'd>>,
        exclusive_end: impl Into<PseudoInt<'m, 'd>>,
    ) -> SetOfIdx<'m, 'd> {
        let data = SetOfIdxData::range(begin, exclusive_end);
        SetOfIdx::new(self, self.set_of_idx.push(data))
    }

    pub(crate) fn add_set_of_idx(&self, data: SetOfIdxData<'d>) -> SetOfIdx<'_, 'd> {
        SetOfIdx::new(self, self.set_of_idx.push(data))
    }

    pub fn set_of_idx_d1<'m>(&'m self, set: SetOfIdx<'m, 'd>) -> SetOfIdxD1<'m, 'd> {
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

    pub fn par_d1<'m>(&'m self, set: SetOfIdx<'m, 'd>) -> ParD1<'m, 'd> {
        let data = ParD1Data::empty(set.fun_ptr());
        ParD1::new(self, self.par_d1.push(data))
    }

    pub fn par_int_d1<'m>(&'m self, set: SetOfIdx<'m, 'd>) -> ParIntD1<'m, 'd> {
        let data = ParIntD1Data::empty(set.fun_ptr());
        ParIntD1::new(self, self.par_int_d1.push(data))
    }

    // idx

    pub(crate) fn idx<'m>(&'m self, set: SetOfIdx<'m, 'd>) -> IdxDepr<'m, 'd> {
        let data = IdxDataDepr::new(set.fun_ptr(), 0);
        IdxDepr::new(self, self.idx.push(data))
    }

    // scalars

    pub(crate) fn add_num<'m>(&'m self, fun: NumFun<'d>) -> Num<'m, 'd> {
        Num::new(self, self.num.push(fun))
    }

    pub(crate) fn add_int<'m>(&'m self, fun: IntFun<'d>) -> Int<'m, 'd> {
        Int::new(self, self.int.push(fun))
    }
}
