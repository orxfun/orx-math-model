use super::model_builder::model_error::ModelError;
use super::model_builder::validation::validate_all_symbol_references;
use super::model_data_ref::ModelDataRef;
use crate::modeling::stages::{Building, Built, ModelStages};
use crate::modeling::symbol_collections::{
    constraints::Constraints, expressions::Expressions, objectives::Objectives,
    parameters::AllParameters, scalars::Scalars, sets::Sets, sums::Sums, terms::Terms,
    variables::AllVariables, vars::Vars,
};
use std::{fmt::Debug, rc::Rc};

pub struct ModelData<B: ModelStages> {
    pub(crate) scalars: Scalars<B>,
    pub(crate) sets: Sets<B>,
    pub(crate) parameters: AllParameters<B>,
    pub(crate) variables: AllVariables<B>,
    pub(crate) vars: Vars<B>,
    pub(crate) terms: Terms<B>,
    pub(crate) expressions: Expressions<B>,
    pub(crate) sums: Sums<B>,
    pub(crate) constraints: Constraints<B>,
    pub(crate) objectives: Objectives<B>,
}

impl<B: ModelStages> Default for ModelData<B> {
    fn default() -> Self {
        Self {
            scalars: Scalars::new(),
            sets: Sets::new(),
            parameters: AllParameters::new(),
            variables: AllVariables::new(),
            vars: Vars::new(),
            terms: Terms::new(),
            expressions: Expressions::new(),
            sums: Sums::new(),
            constraints: Constraints::new(),
            objectives: Objectives::new(),
        }
    }
}

impl ModelData<Building> {
    pub(crate) fn build(
        &self,
        model_data_ref: ModelDataRef<Building>,
    ) -> Result<Rc<ModelData<Built>>, ModelError> {
        validate_all_symbol_references(self, model_data_ref)?;

        let model = Rc::new(ModelData::default());

        self.scalars.add_to_built_model(&model);
        self.sets.add_to_built_model(&model);
        self.parameters.add_to_built_model(&model);
        self.variables.add_to_built_model(&model);
        self.vars.add_to_built_model(&model);
        self.terms.add_to_built_model(&model);
        self.expressions.add_to_built_model(&model);
        self.sums.add_to_built_model(&model);
        self.constraints.add_to_built_model(&model);
        self.objectives.add_to_built_model(&model);

        Ok(model)
    }
}

impl<B: ModelStages> Debug for ModelData<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn debug_log_symbols<S: AsRef<str>, I: Iterator<Item = S>>(
            f: &mut std::fmt::Formatter<'_>,
            which: &str,
            iter: I,
        ) -> std::fmt::Result {
            writeln!(f, "  {}", which)?;
            for (i, item) in iter.enumerate() {
                writeln!(f, "    {}: {}", i, item.as_ref())?;
            }
            Ok(())
        }

        writeln!(f, "ModelData")?;
        debug_log_symbols(f, "Scalars", self.scalars.iter_debug())?;
        debug_log_symbols(f, "Sets", self.sets.iter_debug())?;
        debug_log_symbols(f, "Parameters", self.parameters.iter_debug())?;
        debug_log_symbols(f, "Variables", self.variables.iter_debug())?;
        debug_log_symbols(f, "Vars", self.vars.iter_debug())?;
        debug_log_symbols(f, "Terms", self.terms.iter_debug())?;
        debug_log_symbols(f, "Expressions", self.expressions.iter_debug())?;
        debug_log_symbols(f, "Sums", self.sums.iter_debug())?;
        debug_log_symbols(f, "Constraints", self.constraints.iter_debug())?;
        debug_log_symbols(f, "Objectives", self.objectives.iter_debug())
    }
}
