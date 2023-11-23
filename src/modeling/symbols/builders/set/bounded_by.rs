use super::definition::SetDefinition;
use crate::{
    modeling::stages::Building,
    modeling::{
        reference::HasRef,
        symbol_collections::symbol_collection::SymbolCollection,
        symbols::{
            data::set::{
                bounded_by::BoundedSet,
                from::indep::{IntoSetDataAndDataIndependent, IntoSetDataIndependent},
                set_elements::SetElements,
            },
            set::Set,
        },
    },
};
use std::{ops::Range, rc::Rc};

pub struct SetDefinitionBoundedBy {
    pub(super) definition: SetDefinition,
    pub(super) bounding_set_idx: usize,
    pub(super) bounded_set: BoundedSet,
}

impl SetDefinitionBoundedBy {
    fn set_elements(&self, unbounded_elements: SetElements) -> SetElements {
        SetElements::BoundedBy(
            self.bounding_set_idx,
            Rc::new(unbounded_elements),
            self.bounded_set.clone(),
        )
    }
    fn set(self, unbounded_elements: SetElements) -> Set<Building> {
        let elements = self.set_elements(unbounded_elements);
        self.definition.push_get_set(elements)
    }

    // indep
    pub fn from_range(self, range: Range<usize>) -> Set<Building> {
        let unbounded_elements = SetElements::Range(range);
        self.set(unbounded_elements)
    }
    pub fn from_data<Data>(self, data: Data) -> Set<Building>
    where
        Data: IntoSetDataIndependent,
    {
        let unbounded_elements = data.into_set_elements();
        self.set(unbounded_elements)
    }
    pub fn from_data_and_fun<Data, Fun>(self, data: Rc<Data>, fun: Fun) -> Set<Building>
    where
        (Rc<Data>, Fun): IntoSetDataAndDataIndependent<Data>,
    {
        let unbounded_elements = (data, fun).into_set_elements();
        self.set(unbounded_elements)
    }

    // dep
    pub fn from_set(self, set: Set<Building>) -> Set<Building> {
        let unbounded_elements = set.data().elements.clone();
        self.set(unbounded_elements)
    }
}
