use super::with_dim::ParameterDefinitionWithDimension;
use crate::modeling::{modeler::Modeler, symbols::symbol::Symbol};

pub struct ParameterDefinition {
    pub(crate) modeler: Modeler,
    pub(crate) symbol: Symbol,
}

impl ParameterDefinition {
    pub(crate) fn new(modeler: Modeler, symbol: Symbol) -> Self {
        Self { modeler, symbol }
    }

    pub fn has_dim0(self) -> ParameterDefinitionWithDimension<0> {
        ParameterDefinitionWithDimension { definition: self }
    }
    pub fn has_dim1(self) -> ParameterDefinitionWithDimension<1> {
        ParameterDefinitionWithDimension { definition: self }
    }
    pub fn has_dim2(self) -> ParameterDefinitionWithDimension<2> {
        ParameterDefinitionWithDimension { definition: self }
    }
    pub fn has_dim3(self) -> ParameterDefinitionWithDimension<3> {
        ParameterDefinitionWithDimension { definition: self }
    }
    pub fn has_dim4(self) -> ParameterDefinitionWithDimension<4> {
        ParameterDefinitionWithDimension { definition: self }
    }
}
