use crate::{
    modeling::stages::Building,
    modeling::symbols::{scalar::Scalar, set::Set, term::Term},
};
use std::ops::Mul;

macro_rules! impl_sca {
    ($sca:ty) => {
        impl Mul<Term<Building>> for $sca {
            type Output = Term<Building>;
            fn mul(self, rhs: Term<Building>) -> Self::Output {
                rhs * self
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
