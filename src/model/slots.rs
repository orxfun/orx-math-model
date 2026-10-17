use crate::model::Slot;
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

    pub fn imp_push_get_ptr(&self, value: T) -> *const T {
        self.slots.imp_push_get_ptr(value)
    }

    pub fn push_default(&self) -> Slot<'_, T>
    where
        T: Default,
    {
        Slot::new(self, Default::default())
    }
}
