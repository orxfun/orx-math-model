use crate::symbols::{Set, SetData};
use crate::symbols_d1::{SetD1, SetD1Data};
use orx_imp_vec::{Doubling, ImpVec, SplitVec};

type Pinned<T> = SplitVec<T, Doubling>;
type Bag<T> = ImpVec<T, Pinned<T>>;
fn new_bag<T>() -> Bag<T> {
    Bag::new()
}

pub struct Model<'d> {
    pub set: Bag<SetData<'d>>,
    set_d1: Bag<SetD1Data<'d>>,
}

impl<'d> Model<'d> {
    pub fn new() -> Self {
        Self {
            set: new_bag(),
            set_d1: new_bag(),
        }
    }

    // sets

    pub fn set(&self) -> Set<'_, 'd> {
        Set::new(self.set.push(Default::default()))
    }

    pub fn set_d1<'m>(&'m self, set: Set<'m, 'd>) -> SetD1<'m, 'd> {
        SetD1::new(self.set_d1.push(Default::default()))
    }

    pub(crate) fn sets(&self) -> &Bag<SetData<'d>> {
        &self.set
    }
}
