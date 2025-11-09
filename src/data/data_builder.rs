use crate::symbols::sets::{IndependentSetCollection, SetCore};
use crate::symbols::SetCoreMap;
use crate::SetAndData;
use alloc::boxed::Box;
use alloc::vec::Vec;

#[derive(Default)]
pub struct DataBuilder<'m> {
    set0: SetCoreMap<'m, Box<dyn SetAndData<'m, 0>>>,
    set1: SetCoreMap<'m, Box<dyn SetAndData<'m, 1>>>,
}

impl<'m> DataBuilder<'m> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_sets(mut self, sets: impl IndependentSetCollection<'m>) -> Self {
        for set in sets.into_iter() {
            assert!(!self.contains_set(set), "set is already added");
            // match set.dim() {
            //     0 => self.set0.insert(set, value),
            //     1 => {}
            //     _ => todo!(),
            // }
        }
        self
    }

    fn contains_set(&self, set: SetCore<'m>) -> bool {
        self.set0.contains_key(set) || self.set1.contains_key(set)
    }
}
