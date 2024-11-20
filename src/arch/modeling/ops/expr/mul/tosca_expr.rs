use crate::modeling::ops::expr::core::{get_sums_terms_and_constant, new_expr};
use crate::modeling::stages::Building;
use crate::modeling::{
    reference::HasRef,
    symbols::{conversions::scalar::ToScalar, expression::Expression, scalar::Scalar, set::Set},
};
use std::ops::Mul;

macro_rules! impl_sca {
    ($sca:ty) => {
        impl Mul<Expression<Building>> for $sca {
            type Output = Expression<Building>;
            fn mul(self, rhs: Expression<Building>) -> Self::Output {
                let mul = self.to_scalar_from_host(rhs.sym_ref());

                let (sums, terms, constant) = get_sums_terms_and_constant(rhs.clone());

                let new_constant = constant.map(|x| mul * x);
                let new_terms = terms.iter().map(|x| mul * x);
                let new_sums = sums.iter().map(|x| mul * x);

                new_expr(
                    rhs.sym_ref().core_ref.clone(),
                    new_sums,
                    new_terms,
                    new_constant,
                )
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
