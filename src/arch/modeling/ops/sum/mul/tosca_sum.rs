use crate::modeling::reference::HasRef;
use crate::modeling::symbols::{
    conversions::scalar::ToScalar,
    {scalar::Scalar, set::Set, sum::Sum},
};
use crate::{
    modeling::ops::sum::{
        core::{get_sumoversets_and_expr, new_sum},
        sum_expression::SumExpression,
    },
    modeling::stages::Building,
};
use std::ops::Mul;

macro_rules! impl_sca {
    ($sca:ty) => {
        impl Mul<Sum<Building>> for $sca {
            type Output = Sum<Building>;
            fn mul(self, rhs: Sum<Building>) -> Self::Output {
                let (sum_over_set_indices, expression) = get_sumoversets_and_expr(rhs);
                let core_ref = expression.sym_ref().core_ref.clone();
                let scalar = self.to_scalar(core_ref);

                let new_expression = scalar * expression;

                new_sum(SumExpression::new(sum_over_set_indices, new_expression))
            }
        }
    };
}

impl_sca!(Scalar<Building>);
impl_sca!(Set<Building>);
impl_sca!(i64);
impl_sca!(i32);
impl_sca!(f64);
impl_sca!(f32);
