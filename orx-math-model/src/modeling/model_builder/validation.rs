use crate::modeling::{
    model_data::ModelData,
    model_data_ref::ModelDataRef,
    reference::{HasRef, SymRef},
    stages::Building,
    symbols::{constraint::Constraint, objective::Objective},
};

use super::model_error::ModelError;

pub(crate) fn all_model_references(
    data: &ModelData<Building>,
) -> impl Iterator<Item = &SymRef<Building>> {
    data.sets
        .all_model_references()
        .chain(data.scalars.all_model_references())
        .chain(data.parameters.all_model_references())
        .chain(data.variables.all_model_references())
        .chain(data.vars.all_model_references())
        .chain(data.terms.all_model_references())
        .chain(data.expressions.all_model_references())
        .chain(data.sums.all_model_references())
        .chain(data.constraints.all_model_references())
        .chain(data.objectives.all_model_references())
}

pub(crate) fn validate_model_references_of_constraint(
    model_data_ref: ModelDataRef<Building>,
    constraint: &Constraint<Building>,
) -> Result<(), ModelError> {
    let other = constraint.sym_ref().core_ref;
    if !model_data_ref.is_same_model(&other) {
        Err(ModelError::AlienConstraint(format!("{}", constraint)))
    } else {
        Ok(())
    }
}

pub(crate) fn validate_model_references_of_objective(
    model_data_ref: ModelDataRef<Building>,
    objective: &Objective<Building>,
) -> Result<(), ModelError> {
    let other = objective.sym_ref().core_ref;
    if !model_data_ref.is_same_model(&other) {
        Err(ModelError::AlienObjective(format!("{}", objective)))
    } else {
        Ok(())
    }
}

pub(crate) fn validate_all_symbol_references(
    data: &ModelData<Building>,
    model_data_ref: ModelDataRef<Building>,
) -> Result<(), ModelError> {
    for symref in all_model_references(data) {
        if !symref.core_ref.is_same_model(&model_data_ref) {
            return Err(ModelError::AlienSymbol);
        }
    }
    Ok(())
}
