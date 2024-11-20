use super::{
    model_builder::model_error::ModelError, model_data::ModelData, stages::Building,
    symbols::variable::Variable,
};
use crate::modeling::{
    reference::HasRef, stages::Built, symbol_collections::symbol_collection::SymbolCollection,
};
use std::rc::Rc;

#[derive(Debug)]
pub struct Model {
    data: Rc<ModelData<Built>>,
    constraint_indices: Vec<usize>,
    objective_index: Option<usize>,
}

impl Model {
    pub(crate) fn new(
        data: Rc<ModelData<Built>>,
        constraint_indices: Vec<usize>,
        objective_index: Option<usize>,
    ) -> Self {
        Self {
            data,
            constraint_indices,
            objective_index,
        }
    }

    pub fn cache_var_dim0(
        &self,
        variable: Variable<0, Building>,
    ) -> Result<Variable<0, Built>, ModelError> {
        let data = variable
            .core()
            .variables
            .dim0
            .get(variable.sym_idx())
            .build(&self.data);

        todo!()
    }
}
