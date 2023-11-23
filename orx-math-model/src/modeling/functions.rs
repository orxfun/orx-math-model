use crate::modeling::{
    ops::sum::sum_expression::SumExpression,
    stages::Building,
    symbols::{conversions::forall::ToForAll, data::forall::ForAll, sum::Sum},
};

pub fn sum(sum_expression: SumExpression) -> Sum<Building> {
    super::ops::sum::core::new_sum(sum_expression)
}

pub fn forall<F: ToForAll>(forall_sets: F) -> ForAll<Building> {
    forall_sets.to_forall()
}
