use crate::{
    modeling::stages::Building,
    modeling::symbols::{
        conversions::{constraint_lhs::ConstraintLhs, constraint_rhs::ConstraintRhs},
        data::constraint_expression::ConstraintExpression,
        expression::Expression,
        scalar::Scalar,
        set::Set,
        sum::Sum,
        term::Term,
        var::Var,
        variable::Variable,
    },
};
use std::ops::{Shl, Shr};

// lhs >> rhs
macro_rules! impl_geq_lhs {
    ($lhs:ty) => {
        impl<Rhs: ConstraintRhs> Shr<Rhs> for $lhs {
            type Output = ConstraintExpression<Building>;
            fn shr(self, rhs: Rhs) -> Self::Output {
                self.greater_than_or_eq(rhs)
            }
        }
    };
}
impl_geq_lhs!(Expression<Building>);
impl_geq_lhs!(Sum<Building>);
impl_geq_lhs!(Term<Building>);
impl_geq_lhs!(Var<Building>);
impl_geq_lhs!(Variable<0, Building>);
impl_geq_lhs!(Scalar<Building>);
impl_geq_lhs!(Set<Building>);

// lhs << rhs
macro_rules! impl_leq_lhs {
    ($lhs:ty) => {
        impl<Rhs: ConstraintRhs> Shl<Rhs> for $lhs {
            type Output = ConstraintExpression<Building>;
            fn shl(self, rhs: Rhs) -> Self::Output {
                self.less_than_or_eq(rhs)
            }
        }
    };
}
impl_leq_lhs!(Expression<Building>);
impl_leq_lhs!(Sum<Building>);
impl_leq_lhs!(Term<Building>);
impl_leq_lhs!(Var<Building>);
impl_leq_lhs!(Variable<0, Building>);
impl_leq_lhs!(Scalar<Building>);
impl_leq_lhs!(Set<Building>);
