use std::rc::Rc;

use super::definition::SetDefinition;
use crate::{
    modeling::stages::Building,
    modeling::symbols::{
        data::set::from::dep1::{IntoDepDataAndFunDim1, IntoDepDataDim1},
        set::Set,
    },
};

pub struct SetDefinitionDependsOn {
    pub(super) definition: SetDefinition,
    pub(super) depending_set_idx: usize,
}

impl SetDefinitionDependsOn {
    pub fn from_data<Data>(self, data: Data) -> Set<Building>
    where
        Data: IntoDepDataDim1,
    {
        let elements = data.into_set_elements(self.depending_set_idx);
        self.definition.push_get_set(elements)
    }
    pub fn from_data_and_fun<Data, Fun>(self, data: Rc<Data>, fun: Fun) -> Set<Building>
    where
        (Rc<Data>, Fun): IntoDepDataAndFunDim1<Data>,
    {
        let elements = (data, fun).into_set_elements(self.depending_set_idx);
        self.definition.push_get_set(elements)
    }
}
