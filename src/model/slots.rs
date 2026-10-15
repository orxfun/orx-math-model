use orx_imp_vec::{ImpVec, PinnedVec, SplitVec};

// TODO: to be decided later
type PV<T> = SplitVec<T>;

pub struct Slots<T> {
    slots: ImpVec<T, PV<T>>,
}

impl<T> Slots<T> {
    pub fn new() -> Self {
        Self {
            slots: Default::default(),
        }
    }
}
