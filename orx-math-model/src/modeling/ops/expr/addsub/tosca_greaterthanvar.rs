use super::core;
use crate::{
    modeling::stages::Building,
    modeling::{
        reference::HasRef,
        symbols::{
            conversions::temp_expr::ToTempExpr, expression::Expression, scalar::Scalar, set::Set,
            sum::Sum, term::Term, var::Var, variable::Variable,
        },
    },
};
use std::ops::{Add, Sub};

macro_rules! impl_sca_gtvar {
    ($sca:ty, $gtvar:ty) => {
        impl Add<$gtvar> for $sca {
            type Output = Expression<Building>;
            fn add(self, rhs: $gtvar) -> Self::Output {
                let core_ref = rhs.sym_ref().core_ref;
                let lhs = self.to_temp_expr(core_ref);
                let rhs = rhs.to_temp_expr(core_ref);
                core::add(core_ref, lhs, rhs)
            }
        }
        impl Sub<$gtvar> for $sca {
            type Output = Expression<Building>;
            fn sub(self, rhs: $gtvar) -> Self::Output {
                let core_ref = rhs.sym_ref().core_ref;
                let lhs = self.to_temp_expr(core_ref);
                let rhs = rhs.to_temp_expr(core_ref);
                core::sub(core_ref, lhs, rhs)
            }
        }
    };
}

macro_rules! impl_sca {
    ($sca:ty) => {
        impl_sca_gtvar!($sca, Expression<Building>);
        impl_sca_gtvar!($sca, Sum<Building>);
        impl_sca_gtvar!($sca, Term<Building>);
        impl_sca_gtvar!($sca, Var<Building>);
        impl_sca_gtvar!($sca, Variable<0, Building>);
    };
}

impl_sca!(Scalar<Building>);
impl_sca!(Set<Building>);
impl_sca!(i64);
impl_sca!(i32);
impl_sca!(f64);
impl_sca!(f32);
