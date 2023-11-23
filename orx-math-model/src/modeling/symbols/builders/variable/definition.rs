use super::with_dim::VariableDefinitionDim;
use crate::modeling::{modeler::Modeler, symbols::symbol::Symbol};

pub struct VariableDefinition {
    pub(crate) modeler: Modeler,
    pub(crate) symbol: Symbol,
}

impl VariableDefinition {
    pub(crate) fn new(modeler: Modeler, symbol: Symbol) -> Self {
        Self { modeler, symbol }
    }

    pub fn has_dim0(self) -> VariableDefinitionDim<0> {
        VariableDefinitionDim { definition: self }
    }
    pub fn has_dim1(self) -> VariableDefinitionDim<1> {
        VariableDefinitionDim { definition: self }
    }
    pub fn has_dim2(self) -> VariableDefinitionDim<2> {
        VariableDefinitionDim { definition: self }
    }
    pub fn has_dim3(self) -> VariableDefinitionDim<3> {
        VariableDefinitionDim { definition: self }
    }
    pub fn has_dim4(self) -> VariableDefinitionDim<4> {
        VariableDefinitionDim { definition: self }
    }
}
