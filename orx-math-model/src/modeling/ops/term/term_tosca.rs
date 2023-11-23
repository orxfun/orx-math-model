use super::core::{self, get_term_coef_var};
use crate::{
    modeling::stages::Building,
    modeling::{
        reference::HasRef,
        symbols::{conversions::scalar::ToScalar, term::Term},
    },
};
use std::ops::{Div, Mul};

impl<S: ToScalar> Mul<S> for Term<Building> {
    type Output = Term<Building>;
    fn mul(self, rhs: S) -> Self::Output {
        let (coef, var) = get_term_coef_var(self);
        core::new_term(var, coef * rhs.to_scalar_from_host(self.sym_ref()))
    }
}
impl<S: ToScalar> Div<S> for Term<Building> {
    type Output = Term<Building>;
    fn div(self, rhs: S) -> Self::Output {
        let (coef, var) = get_term_coef_var(self);
        core::new_term(var, coef / rhs.to_scalar_from_host(self.sym_ref()))
    }
}
