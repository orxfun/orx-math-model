use crate::symbols::{Idx, IdxData, Set, SetData};
use crate::symbols_d1::{SetD1, SetD1Data};
use orx_imp_vec::{Doubling, ImpVec, SplitVec};

type Pinned<T> = SplitVec<T, Doubling>;
type Bag<T> = ImpVec<T, Pinned<T>>;
fn new_bag<T>() -> Bag<T> {
    Bag::new()
}

pub struct Model<'d> {
    pub(crate) set: Bag<SetData<'d>>,
    set_d1: Bag<SetD1Data<'d>>,

    pub(crate) idx: Bag<IdxData>,
}

impl<'d> Model<'d> {
    pub fn new() -> Self {
        Self {
            set: new_bag(),
            set_d1: new_bag(),

            idx: new_bag(),
        }
    }

    // sets

    pub fn set(&self) -> Set<'_, 'd> {
        self.add_set(SetData::default())
    }

    pub(crate) fn add_set<'m>(&'m self, data: SetData<'d>) -> Set<'m, 'd> {
        Set::new(self, self.set.push(data))
    }

    pub fn set_d1<'m>(&'m self, set: Set<'m, 'd>) -> SetD1<'m, 'd> {
        SetD1::new(self, self.set_d1.push(SetD1Data::default(set)))
    }

    // idx

    pub(crate) fn idx<'m>(&'m self, set: Set<'m, 'd>) -> Idx<'m, 'd> {
        Idx::new(self, self.idx.push(IdxData::new(set)))
    }
}
