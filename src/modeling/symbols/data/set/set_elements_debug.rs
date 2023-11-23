use super::{next_prev_of::NextPrevBound, set_elements::SetElements};
use crate::modeling::{
    model_data::ModelData, stages::ModelStages,
    symbol_collections::symbol_collection::SymbolCollection,
};
use std::fmt::Debug;

pub(crate) struct SetElementsAndCore<'a, B: ModelStages> {
    set_elements: &'a SetElements,
    core: &'a ModelData<B>,
}
impl<'a, B: ModelStages> SetElementsAndCore<'a, B> {
    pub(crate) fn new(core: &'a ModelData<B>, set_elements: &'a SetElements) -> Self {
        Self { set_elements, core }
    }
}
impl<'a, B: ModelStages> Debug for SetElementsAndCore<'a, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.set_elements.core_debug(self.core, f)
    }
}

impl SetElements {
    fn core_debug<B: ModelStages>(
        &self,
        core: &ModelData<B>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let depkey = |i: &usize| &core.sets.get(*i).key;

        match self {
            Self::Range(arg0) => f.debug_tuple("Range").field(arg0).finish(),
            Self::Vec(arg0) => f.debug_struct("Vec").field("len", &arg0.len()).finish(),
            Self::Set(arg0) => f.debug_struct("Set").field("len", &arg0.len()).finish(),
            Self::DataAndFun(_) => f.debug_tuple("DataAndFun").finish(),
            Self::Dep1DataAndFun(arg0, _) => f
                .debug_struct("Dep1DataAndFun")
                .field("depends_on", depkey(arg0))
                .finish(),
            Self::Dep1VecVec(arg0, arg1) => f
                .debug_struct("Dep1VecVec")
                .field("depends_on", depkey(arg0))
                .field("outer_len", &arg1.len())
                .finish(),
            Self::Dep1VecSet(arg0, arg1) => f
                .debug_struct("Dep1VecSet")
                .field("depends_on", depkey(arg0))
                .field("outer_len", &arg1.len())
                .finish(),
            Self::Dep1HashMapVec(arg0, arg1) => f
                .debug_struct("Dep1HashMapVec")
                .field("depends_on", depkey(arg0))
                .field("outer_len", &arg1.len())
                .finish(),
            Self::Dep1HashMapSet(arg0, arg1) => f
                .debug_struct("Dep1HashMapSet")
                .field("depends_on", depkey(arg0))
                .field("outer_len", &arg1.len())
                .finish(),
            Self::Dep1BTreeMapVec(arg0, arg1) => f
                .debug_struct("Dep1BTreeMapVec")
                .field("depends_on", depkey(arg0))
                .field("outer_len", &arg1.len())
                .finish(),
            Self::Dep1BTreeMapSet(arg0, arg1) => f
                .debug_struct("Dep1BTreeMapSet")
                .field("depends_on", depkey(arg0))
                .field("outer_len", &arg1.len())
                .finish(),
            Self::NextPrevOf(arg0, arg1) => match arg1 {
                NextPrevBound::NextUntilInclusive(until) => {
                    if until == &usize::MAX {
                        f.debug_struct("NextOf")
                            .field("next_of", depkey(arg0))
                            .finish()
                    } else {
                        f.debug_struct("NextOf")
                            .field("next_of", depkey(arg0))
                            .field("less_than_or_equal_to", until)
                            .finish()
                    }
                }
                NextPrevBound::PrevAfterInclusive(after) => {
                    if after == &0 {
                        f.debug_struct("PreviousOf")
                            .field("prev_of", depkey(arg0))
                            .finish()
                    } else {
                        f.debug_struct("NextOf")
                            .field("prev_of", depkey(arg0))
                            .field("greater_than_or_equal_to", after)
                            .finish()
                    }
                }
            },
            Self::BoundedBy(arg0, arg1, arg2) => f
                .debug_struct("BoundedBy")
                .field("bounding_set", depkey(arg0))
                .field("bounding_relation", &arg2.get_relation_str(depkey(arg0)))
                .field("unbounded_set", &SetElementsAndCore::new(core, arg1))
                .finish(),
            Self::OtherThan(arg0, arg1) => f
                .debug_struct("OtherThan")
                .field("other_than_set", depkey(arg0))
                .field("unbounded_set", &SetElementsAndCore::new(core, arg1))
                .finish(),
        }
    }
}
