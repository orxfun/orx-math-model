use crate::modeling::symbol_collections::symbol_collection::SymbolCollection;
use crate::{
    modeling::stages::Building,
    modeling::{
        reference::HasRef,
        symbols::{
            conversions::constraint_lhs::ConstraintLhs,
            data::{
                objective_data::ObjectiveData,
                objective_direction::{maximize, minimize, ObjectiveDirectionMarker},
            },
            objective::Objective,
        },
    },
};
use std::ops::BitOr;

fn new_objective<E: ConstraintLhs, Dir: ObjectiveDirectionMarker>(
    expression: E,
    direction: Dir,
) -> Objective<Building> {
    let direction = direction.direction();
    let expression = expression.to_lhs_expression();
    let expression_ref = *expression.sym_ref();
    let core_ref = expression.sym_ref().core_ref;
    let symref = core_ref.new_symref_objective();
    let symbol = Objective::new(symref);

    let storage = ObjectiveData {
        symbol,
        direction,
        expression_ref,
    };
    core_ref.core().objectives.push(storage)
}

impl<E: ConstraintLhs> BitOr<E> for minimize {
    type Output = Objective<Building>;
    fn bitor(self, rhs: E) -> Self::Output {
        new_objective(rhs, self)
    }
}

impl<E: ConstraintLhs> BitOr<E> for maximize {
    type Output = Objective<Building>;
    fn bitor(self, rhs: E) -> Self::Output {
        new_objective(rhs, self)
    }
}
