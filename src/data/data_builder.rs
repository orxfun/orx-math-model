use crate::data::set_data::SetDataCollection;
use crate::symbols::sets::IndependentSetCollection;
use crate::symbols::SetCoreMap;
use crate::SetAndData;
use alloc::boxed::Box;

#[derive(Default)]
pub struct DataBuilder<'m> {
    sets: SetCoreMap<'m, Box<dyn SetAndData<'m> + 'm>>,
}

impl<'m> DataBuilder<'m> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_sets(mut self, sets: impl SetDataCollection<'m>) -> Self {
        for set_and_data in sets.into_iter() {
            let set = set_and_data.set();
            assert!(!self.sets.contains_key(set), "set is already added");
            self.sets.insert(set, set_and_data);
        }
        self
    }
}
