use super::sum_expression::SumExpression;
use crate::{
    modeling::stages::Building,
    modeling::symbols::{
        conversions::{sum_over_sets::ToSumOverSetIndices, temp_expr::ToTempExpr},
        expression::Expression,
        scalar::Scalar,
        set::Set,
        sum::Sum,
        term::Term,
        var::Var,
        variable::Variable,
    },
};
use std::ops::BitOr;

macro_rules! impl_toexpr_tosets {
    ($expr:ty, $sets:ty) => {
        impl BitOr<$expr> for $sets {
            type Output = SumExpression;
            fn bitor(self, rhs: $expr) -> Self::Output {
                let core_ref = self.core_ref();
                let sum_over_set_indices = self.sum_over_sets();
                let temp_expr = rhs.to_temp_expr(core_ref.clone());
                let expression = temp_expr.to_expr(core_ref);
                SumExpression::new(sum_over_set_indices, expression)
            }
        }
    };
}

macro_rules! impl_toexpr {
    ($expr:ty) => {
        impl_toexpr_tosets!($expr, Set<Building>);
        impl_toexpr_tosets!($expr, (Set<Building>, Set<Building>));
        impl_toexpr_tosets!($expr, (Set<Building>, Set<Building>, Set<Building>));
        impl_toexpr_tosets!(
            $expr,
            (Set<Building>, Set<Building>, Set<Building>, Set<Building>)
        );
    };
}

impl_toexpr!(Expression<Building>);
impl_toexpr!(Sum<Building>);
impl_toexpr!(Term<Building>);
impl_toexpr!(Var<Building>);
impl_toexpr!(Variable<0, Building>);
impl_toexpr!(Scalar<Building>);
impl_toexpr!(Set<Building>);
