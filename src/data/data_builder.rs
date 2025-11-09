use crate::data::data::Data;
use crate::data::set_data::SetDataCollection;
use crate::symbols::{SetCoreMap, Symbol};
use crate::{Model, SetAndData};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use orx_iterable::Collection;

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
            assert!(!self.sets.contains_symbol(set), "set is already added");
            self.sets.insert(set, set_and_data);
        }
        self
    }

    pub fn finish(self) -> Result<Data<'m>, String> {
        // TODO: proper error type

        let m = self.model;

        let symbols = m.data.sets.iter();
        let keys = symbols.map(|s| (Symbol::key(s), s));
        let missing = keys.filter(|(key, _)| !self.sets.contains_key(*key));
        for (_, set) in missing {
            return Err(format!("missing data for set with key {}", set.key()));
        }

        let data = Data::new(self.model, self.sets);
        Ok(data)
    }
}
