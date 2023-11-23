use crate::modeling::symbol_collections::symbol_collection::SymbolCollection;
use crate::{
    modeling::stages::Building,
    modeling::{
        reference::HasRef,
        symbols::{
            constraint::Constraint,
            data::{
                constraint_data::ConstraintData, constraint_expression::ConstraintExpression,
                forall::ForAll,
            },
        },
    },
};
use std::ops::BitOr;

pub(crate) fn new_constraint(
    constraint_expression: ConstraintExpression<Building>,
    forall: ForAll<Building>,
) -> Constraint<Building> {
    let core_ref = constraint_expression.core_ref;
    let symref = core_ref.new_symref_constraint();
    let symbol = Constraint::new(symref);
    let storage = ConstraintData {
        symbol,
        constraint_expression,
        forall,
    };
    core_ref.core().constraints.push(storage)
}

impl BitOr<ForAll<Building>> for ConstraintExpression<Building> {
    type Output = Constraint<Building>;
    fn bitor(self, forall: ForAll<Building>) -> Self::Output {
        new_constraint(self, forall)
    }
}
