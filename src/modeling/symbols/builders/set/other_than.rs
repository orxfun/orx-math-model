use super::definition::SetDefinition;
use crate::modeling::{
    reference::HasRef,
    stages::Building,
    symbol_collections::symbol_collection::SymbolCollection,
    symbols::{
        data::set::{
            from::indep::{IntoSetDataAndDataIndependent, IntoSetDataIndependent},
            set_elements::SetElements,
        },
        set::Set,
    },
};
use std::{ops::Range, rc::Rc};

pub struct SetDefinitionOtherThan {
    pub(super) definition: SetDefinition,
    pub(super) other_than_set_idx: usize,
}

impl SetDefinitionOtherThan {
    fn set_elements(&self, unbounded_elements: SetElements) -> SetElements {
        SetElements::OtherThan(self.other_than_set_idx, Rc::new(unbounded_elements))
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
