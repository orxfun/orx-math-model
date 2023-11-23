use crate::modeling::{
    reference::HasRef,
    symbols::{conversions::scalar::ToScalar, sum::Sum},
};
use crate::{
    modeling::ops::sum::{
        core::{get_sumoversets_and_expr, new_sum},
        sum_expression::SumExpression,
    },
    modeling::stages::Building,
};
use std::ops::Mul;

impl<S: ToScalar> Mul<S> for Sum<Building> {
    type Output = Sum<Building>;
    fn mul(self, rhs: S) -> Self::Output {
        let (sum_over_set_refs, expression) = get_sumoversets_and_expr(self);
        let core_ref = expression.sym_ref().core_ref;
        let scalar = rhs.to_scalar(core_ref);

        let new_expression = expression * scalar;

        new_sum(SumExpression::new(sum_over_set_refs, new_expression))
    }
}
