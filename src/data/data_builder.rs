use crate::data::data::Data;
use crate::data::par_data::{ParAndData, ParDataCollection};
use crate::data::set_data::SetDataCollection;
use crate::symbols::{ParCoreMap, SetCoreMap, Symbol};
use crate::{Model, SetAndData};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use orx_iterable::Collection;

pub struct DataBuilder<'m> {
    model: &'m Model,
    sets: Vec<Box<dyn SetAndData<'m> + 'm>>,
    pars: Vec<Box<dyn ParAndData<'m> + 'm>>,
}

impl<'m> DataBuilder<'m> {
    pub fn new(model: &'m Model) -> Self {
        let sets = Vec::new();
        let pars = Vec::new();
        Self { model, sets, pars }
    }

    pub fn sets(mut self, sets: impl SetDataCollection<'m>) -> Self {
        self.sets.extend(sets.into_iter());
        self
    }

    pub fn pars(mut self, pars: impl ParDataCollection<'m>) -> Self {
        self.pars.extend(pars.into_iter());
        self
    }

    pub fn finish(self) -> Result<Data<'m>, String> {
        // TODO: proper error type
        let m = self.model;

        // sets
        let mut sets = SetCoreMap::new();
        for set_and_data in self.sets {
            let set = set_and_data.set();
            let inserted = sets.try_insert(set, set_and_data);
            if !inserted {
                return Err(format!(
                    "double data definition for set with key {}",
                    *set.symbol().symbol.key
                ));
            }
        }

        let symbols = m.data.sets.iter();
        let keys = symbols.map(|s| (Symbol::addr(s), s));
        let missing = keys.filter(|(key, _)| !sets.contains_key(*key));
        // TODO: report all missing elements at once
        #[allow(clippy::never_loop)]
        for (_, set) in missing {
            return Err(format!("missing data for set with key {}", *set.key));
        }

        // pars
        let mut pars = ParCoreMap::new();
        for par_and_data in self.pars {
            let par = par_and_data.par();
            let inserted = pars.try_insert(par, par_and_data);
            if !inserted {
                return Err(format!(
                    "double data definition for par with key {}",
                    *par.symbol().symbol.key
                ));
            }
        }

        let symbols = m.data.pars.iter();
        let keys = symbols.map(|s| (Symbol::addr(s), s));
        let missing = keys.filter(|(key, _)| !pars.contains_key(*key));
        // TODO: report all missing elements at once
        #[allow(clippy::never_loop)]
        for (_, par) in missing {
            return Err(format!("missing data for par with key {}", *par.key));
        }

        let data = Data::new(self.model, sets);
        Ok(data)
    }
}
