use crate::symbols::sets::IndependentSetCollection;
use crate::symbols::SetCoreMap;
use crate::SetAndData;
use alloc::boxed::Box;

#[derive(Default)]
pub struct DataBuilder<'m> {
    sets: SetCoreMap<'m, Box<dyn SetAndData<'m>>>,
}

impl<'m> DataBuilder<'m> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_sets(mut self, sets: impl IndependentSetCollection<'m>) -> Self {
        for set in sets.into_iter() {
            assert!(!self.sets.contains_key(set), "set is already added");
            // self.sets.insert(set, value);
        }
        self
    }
}
