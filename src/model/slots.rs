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

    pub fn imp_push_get_ref(&self, value: T) -> &T {
        self.slots.imp_push_get_ref(value)
    }

    pub fn push_default(&self) -> &T
    where
        T: Default,
    {
        self.slots.imp_push_get_ref(Default::default())
    }

    pub unsafe fn set(&self, element: &T, new_value: T) {
        debug_assert!(self.slots.index_of(element).is_some());
    }
}
