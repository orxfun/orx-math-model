use super::core::{get_term_coef_var, new_term};
use crate::{
    modeling::stages::Building,
    modeling::{
        ops::scalar::core::new_scalar,
        reference::HasRef,
        symbols::{
            conversions::var::ToVar, data::scalar_data::ScalarVariant, term::Term, var::Var,
            variable::Variable,
        },
    },
};
use std::ops::Neg;

impl Neg for Term<Building> {
    type Output = Term<Building>;
    fn neg(self) -> Self::Output {
        let (old_coef, var) = get_term_coef_var(self);
        new_term(var, -old_coef)
    }
}

macro_rules! impl_var {
    ($var:ty) => {
        impl Neg for $var {
            type Output = Term<Building>;
            fn neg(self) -> Self::Output {
                let variant = ScalarVariant::new_from_num(-1.0);
                let minus_one = new_scalar(self.sym_ref().core_ref.clone(), variant);
                new_term(self.to_var(), minus_one)
            }
        }
    };
}

impl_var!(Var<Building>);
impl_var!(Variable<0, Building>);
