use super::core;
use crate::{
    modeling::stages::Building,
    modeling::{
        reference::HasRef,
        symbols::{
            conversions::{scalar::ToScalar, var::ToVar},
            term::Term,
            var::Var,
            variable::Variable,
        },
    },
};
use std::ops::{Div, Mul};

macro_rules! impl_var {
    ($var:ty) => {
        impl<S: ToScalar> Mul<S> for $var {
            type Output = Term<Building>;
            fn mul(self, rhs: S) -> Self::Output {
                core::new_term(self.to_var(), rhs.to_scalar_from_host(self.sym_ref()))
            }
        }
        impl<S: ToScalar> Div<S> for $var {
            type Output = Term<Building>;
            fn div(self, rhs: S) -> Self::Output {
                core::new_term(self.to_var(), 1.0 / rhs.to_scalar_from_host(self.sym_ref()))
            }
        }
    };
}

impl_var!(Var<Building>);
impl_var!(Variable<0, Building>);
