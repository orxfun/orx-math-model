use super::core::new_term;
use crate::{
    modeling::stages::Building,
    modeling::{
        reference::HasRef,
        symbols::{
            conversions::{scalar::ToScalar, var::ToVar},
            scalar::Scalar,
            set::Set,
            term::Term,
            var::Var,
            variable::Variable,
        },
    },
};
use std::ops::Mul;

macro_rules! impl_sca_var {
    ($sca:ty, $var:ty) => {
        impl Mul<$var> for $sca {
            type Output = Term<Building>;
            fn mul(self, rhs: $var) -> Self::Output {
                let var = rhs.to_var();
                new_term(var.clone(), self.to_scalar_from_host(var.sym_ref()))
            }
        }
    };
}

macro_rules! impl_sca {
    ($sca:ty) => {
        impl_sca_var!($sca, Var<Building>);
        impl_sca_var!($sca, Variable<0, Building>);
    };
}

impl_sca!(Scalar<Building>);
impl_sca!(Set<Building>);
impl_sca!(i64);
impl_sca!(i32);
impl_sca!(f64);
impl_sca!(f32);
