use orx_imp_vec::{Doubling, ImpVec, SplitVec};

type Pinned<T> = SplitVec<T, Doubling>;

pub type Bag<T> = ImpVec<T, Pinned<T>>;

pub fn new_bag<T>() -> Bag<T> {
    Bag::new()
}
