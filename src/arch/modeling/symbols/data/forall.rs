use crate::modeling::model_data::ModelData;
use crate::modeling::stages::{Building, Built, ModelStages};
use crate::modeling::symbol_collections::symbol_collection::SymbolCollection;
use crate::{data_structs::forall_set_refs::ForAllSetRefs, modeling::model_data_ref::ModelDataRef};
use std::fmt::{Debug, Display};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ForAll<B: ModelStages> {
    pub(crate) sets: ForAllSetRefs<B>,
}
impl ForAll<Building> {
    pub(crate) fn build(&self, model: &Rc<ModelData<Built>>) -> ForAll<Built> {
        ForAll {
            sets: self.sets.build(model),
        }
    }
}

pub(crate) struct ForAllAndCoreRef<'a, B: ModelStages>(
    pub(crate) &'a ForAll<B>,
    pub(crate) ModelDataRef<B>,
);

impl<'a, B: ModelStages> Display for ForAllAndCoreRef<'a, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sets = &self.1.core().sets;
        let set_keys: Vec<_> = self
            .0
            .sets
            .iter()
            .map(|x| sets.get(x.sym_idx).key.as_str())
            .collect();
        write!(f, "forall {}", set_keys.join(","))
    }
}
impl<'a, B: ModelStages> Debug for ForAllAndCoreRef<'a, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sets = &self.1.core().sets;
        let set_keys: Vec<_> = self
            .0
            .sets
            .iter()
            .map(|x| sets.get(x.sym_idx).key.as_str())
            .collect();
        f.debug_struct("ForAll").field("sets", &set_keys).finish()
    }
}
