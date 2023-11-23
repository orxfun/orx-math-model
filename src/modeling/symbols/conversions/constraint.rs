use crate::{
    modeling::stages::Building,
    modeling::symbols::{
        constraint::Constraint, data::constraint_expression::ConstraintExpression,
    },
};

impl From<ConstraintExpression<Building>> for Constraint<Building> {
    fn from(value: ConstraintExpression<Building>) -> Self {
        value.as_single_constraint()
    }
}
