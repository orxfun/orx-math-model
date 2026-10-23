use crate::symbols::{Set, SetData};
use orx_imp_vec::{Doubling, ImpVec, SplitVec};

type Pinned<T> = SplitVec<T, Doubling>;
type Bag<T> = ImpVec<T, Pinned<T>>;

pub struct Model<'d> {
    set: Bag<SetData<'d>>,
}

impl<'d> Model<'d> {
    pub fn new() -> Self {
        Self { set: Bag::new() }
    }

    // sets

    pub fn set(&self) -> Set<'_, 'd> {
        Set::new(self.set.push(Default::default()))
    }
}
