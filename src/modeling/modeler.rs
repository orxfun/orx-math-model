use super::{
    model_builder::{defining_stages::AddingConstraints, model_definition::ModelDefinition},
    model_data::ModelData,
    model_data_ref::ModelDataRef,
    reference::SymRef,
    symbol_collections::symbol_collection::SymbolCollection,
    symbols::{
        builders::{
            parameter::definition::ParameterDefinition, set::definition::SetDefinition,
            variable::definition::VariableDefinition,
        },
        symbol::Symbol,
    },
};
use crate::modeling::stages::Building;
use std::rc::Rc;

pub(crate) enum SymbolFromBuilder {
    Set,
    Parameter { dim: usize },
    Variable { dim: usize },
}

#[derive(Clone, Debug)]
pub struct Modeler {
    pub(super) data: Rc<ModelData<Building>>,
}
impl Modeler {
    pub fn new() -> Self {
        let data = Rc::new(ModelData::default());

        let ptr_core = Rc::as_ptr(&data);
        let model_reference = Building(ptr_core);
        let core_ref = ModelDataRef { model_reference };
        data.scalars.add_common_constants(core_ref);

        Self { data }
    }
    pub fn define(&self) -> ModelDefinition<AddingConstraints> {
        ModelDefinition::new(self.data.clone())
    }
}
impl Default for Modeler {
    fn default() -> Self {
        Self::new()
    }
}

impl Modeler {
    // grow
    pub(crate) fn new_index(&self, sym: SymbolFromBuilder) -> SymRef<Building> {
        let sym_idx = match sym {
            SymbolFromBuilder::Set => self.data.sets.len(),
            SymbolFromBuilder::Parameter { dim } => match dim {
                0 => self.data.parameters.dim0.len(),
                1 => self.data.parameters.dim1.len(),
                2 => self.data.parameters.dim2.len(),
                3 => self.data.parameters.dim3.len(),
                4 => self.data.parameters.dim4.len(),
                _ => panic!("nono dim"),
            },
            SymbolFromBuilder::Variable { dim } => match dim {
                0 => self.data.variables.dim0.len(),
                1 => self.data.variables.dim1.len(),
                2 => self.data.variables.dim2.len(),
                3 => self.data.variables.dim3.len(),
                4 => self.data.variables.dim4.len(),
                _ => panic!("nono dim"),
            },
        };

        SymRef::new(&self.data, sym_idx)
    }

    // new
    pub fn set<S: Into<Symbol>>(&self, symbol: S) -> SetDefinition {
        SetDefinition::new(self.clone(), symbol.into())
    }
    pub fn parameter<S: Into<Symbol>>(&self, symbol: S) -> ParameterDefinition {
        ParameterDefinition::new(self.clone(), symbol.into())
    }
    pub fn variable<S: Into<Symbol>>(&self, symbol: S) -> VariableDefinition {
        VariableDefinition::new(self.clone(), symbol.into())
    }
}
