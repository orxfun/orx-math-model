use crate::data::data::Data;
use crate::data::set_data::SetDataCollection;
use crate::symbols::SetCoreMap;
use crate::{Model, SetAndData};
use alloc::boxed::Box;

pub struct DataBuilder<'m> {
    model: &'m Model,
    sets: SetCoreMap<'m, Box<dyn SetAndData<'m> + 'm>>,
}

impl<'m> DataBuilder<'m> {
    pub fn new(model: &'m Model) -> Self {
        Self {
            model,
            sets: Default::default(),
        }
    }

    pub fn sets(mut self, sets: impl SetDataCollection<'m>) -> Self {
        for set_and_data in sets.into_iter() {
            let set = set_and_data.set();
            assert!(!self.sets.contains_key(set), "set is already added");
            self.sets.insert(set, set_and_data);
        }
        self
    }

    pub fn finish(self) -> Result<Data<'m>, &'static str> {
        // TODO: proper error type

        //
    }
}
