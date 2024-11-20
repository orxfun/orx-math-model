use super::{
    defining_stages::{AddingConstraints, DefiningStage, ModelDefined},
    model_error::ModelError,
    validation::{validate_model_references_of_constraint, validate_model_references_of_objective},
};
use crate::modeling::{
    model::Model,
    model_data::ModelData,
    model_data_ref::ModelDataRef,
    reference::HasRef,
    stages::{Building, Built},
    symbols::{constraint::Constraint, objective::Objective, variable::Variable},
};
use std::{marker::PhantomData, rc::Rc};

pub struct ModelDefinition<S: DefiningStage> {
    data_building: Rc<ModelData<Building>>,
    constraints: Vec<Constraint<Building>>,
    objective: Option<Objective<Building>>,
    ph: PhantomData<S>,
    data_built: Option<Rc<ModelData<Built>>>,
    data_built_error: Option<ModelError>,
}

impl<S: DefiningStage> ModelDefinition<S> {
    fn modref_building(&self) -> ModelDataRef<Building> {
        ModelDataRef {
            model_reference: Building(Rc::as_ptr(&self.data_building)),
        }
    }
}

impl ModelDefinition<AddingConstraints> {
    fn define(mut self) -> ModelDefinition<ModelDefined> {
        let modref = self.modref_building();
        match self.data_building.build(modref) {
            Ok(data_built) => {
                self.data_built = Some(data_built);
            }
            Err(data_built_error) => {
                self.data_built_error = Some(data_built_error);
            }
        }
        ModelDefinition {
            data_building: self.data_building,
            constraints: self.constraints,
            objective: self.objective,
            ph: Default::default(),
            data_built: self.data_built,
            data_built_error: self.data_built_error,
        }
    }

    pub(crate) fn new(data_building: Rc<ModelData<Building>>) -> Self {
        Self {
            data_building,
            constraints: Default::default(),
            objective: Default::default(),
            ph: Default::default(),
            data_built: None,
            data_built_error: None,
        }
    }

    pub fn constraints<const N: usize, C>(mut self, constraints: [C; N]) -> Self
    where
        C: Into<Constraint<Building>>,
    {
        for c in constraints {
            self.constraints.push(c.into());
        }
        self
    }
    pub fn constraint<C>(mut self, constraint: C) -> Self
    where
        C: Into<Constraint<Building>>,
    {
        self.constraints.push(constraint.into());
        self
    }

    pub fn objective(mut self, objective: Objective<Building>) -> ModelDefinition<ModelDefined> {
        self.objective = Some(objective);
        self.define()
    }

    pub fn minimize0(self) -> ModelDefinition<ModelDefined> {
        self.define()
    }
}

impl ModelDefinition<ModelDefined> {
    fn is_data_successfully_built(&self) -> Result<(), ModelError> {
        if let Some(error) = &self.data_built_error {
            Err(error.clone())
        } else {
            debug_assert!(self.data_built.is_some());
            Ok(())
        }
    }
    fn is_variable_in_same_model<const D: usize>(
        &self,
        variable: &Variable<D, Building>,
    ) -> Result<(), ModelError> {
        self.is_data_successfully_built()?;
        if !variable
            .sym_ref()
            .core_ref
            .is_same_model(&self.modref_building())
        {
            Err(ModelError::AlienVariable(format!("{}", variable)))
        } else {
            Ok(())
        }
    }
    pub fn cache_var_dim0(
        &self,
        variable: Variable<0, Building>,
    ) -> Result<Variable<0, Built>, ModelError> {
        self.is_variable_in_same_model(&variable)?;
        let built_data = variable.data().build(self.data_built.as_ref().unwrap());
        Ok(built_data.symbol)
    }
    pub fn cache_var_dim1(
        &self,
        variable: Variable<1, Building>,
    ) -> Result<Variable<1, Built>, ModelError> {
        self.is_variable_in_same_model(&variable)?;
        let built_data = variable.data().build(self.data_built.as_ref().unwrap());
        Ok(built_data.symbol)
    }
    pub fn cache_var_dim2(
        &self,
        variable: Variable<2, Building>,
    ) -> Result<Variable<2, Built>, ModelError> {
        self.is_variable_in_same_model(&variable)?;
        let built_data = variable.data().build(self.data_built.as_ref().unwrap());
        Ok(built_data.symbol)
    }
    pub fn cache_var_dim3(
        &self,
        variable: Variable<3, Building>,
    ) -> Result<Variable<3, Built>, ModelError> {
        self.is_variable_in_same_model(&variable)?;
        let built_data = variable.data().build(self.data_built.as_ref().unwrap());
        Ok(built_data.symbol)
    }
    pub fn cache_var_dim4(
        &self,
        variable: Variable<4, Building>,
    ) -> Result<Variable<4, Built>, ModelError> {
        self.is_variable_in_same_model(&variable)?;
        let built_data = variable.data().build(self.data_built.as_ref().unwrap());
        Ok(built_data.symbol)
    }

    pub fn build(self) -> Result<Model, ModelError> {
        self.is_data_successfully_built()?;

        if self.constraints.is_empty() && self.objective.is_none() {
            Err(ModelError::EmptyModel)
        } else {
            let modref = self.modref_building();

            for c in &self.constraints {
                validate_model_references_of_constraint(modref, c)?;
            }
            if let Some(o) = &self.objective {
                validate_model_references_of_objective(modref, o)?;
            }

            let constraint_indices = self.constraints.iter().map(|x| x.sym_idx()).collect();
            let objective_index = self.objective.map(|x| x.sym_idx());

            Ok(Model::new(
                self.data_built.unwrap(),
                constraint_indices,
                objective_index,
            ))
        }
    }
}
