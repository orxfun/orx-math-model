use super::{
    core::{get_sumoversets_and_expr, new_sum},
    sum_expression::SumExpression,
};
use crate::{modeling::stages::Building, modeling::symbols::sum::Sum};
use std::ops::Neg;

impl Neg for Sum<Building> {
    type Output = Sum<Building>;
    fn neg(self) -> Self::Output {
        let (sum_over_set_refs, expression) = get_sumoversets_and_expr(self);
        let neg_expression = -expression;
        new_sum(SumExpression::new(sum_over_set_refs, neg_expression))
    }
}
