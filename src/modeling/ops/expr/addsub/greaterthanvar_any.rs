use super::core;
use crate::{
    modeling::stages::Building,
    modeling::{
        reference::HasRef,
        symbols::{
            conversions::temp_expr::ToTempExpr, expression::Expression, sum::Sum, term::Term,
            var::Var, variable::Variable,
        },
    },
};
use std::ops::{Add, Sub};

macro_rules! impl_gt_var {
    ($termexpr:ty) => {
        impl<T: ToTempExpr> Add<T> for $termexpr {
            type Output = Expression<Building>;
            fn add(self, rhs: T) -> Self::Output {
                let core_ref = self.sym_ref().core_ref;
                let lhs = self.to_temp_expr(core_ref);
                let rhs = rhs.to_temp_expr(core_ref);
                core::add(core_ref, lhs, rhs)
            }
        }

        impl<T: ToTempExpr> Sub<T> for $termexpr {
            type Output = Expression<Building>;
            fn sub(self, rhs: T) -> Self::Output {
                let core_ref = self.sym_ref().core_ref;
                let lhs = self.to_temp_expr(core_ref);
                let rhs = rhs.to_temp_expr(core_ref);
                core::sub(core_ref, lhs, rhs)
            }
        }
    };
}

impl_gt_var!(Expression<Building>);
impl_gt_var!(Sum<Building>);
impl_gt_var!(Term<Building>);
impl_gt_var!(Var<Building>);
impl_gt_var!(Variable<0, Building>);
