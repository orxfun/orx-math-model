use super::sca_sca_core;
use crate::{
    modeling::stages::Building,
    modeling::symbols::{
        conversions::scalar::AsScalar, parameter::Parameter, scalar::Scalar, set::Set,
    },
};
use std::ops::{Add, Div, Mul, Sub};

macro_rules! impl_sca_sca_ops {
    ($x:ty, $y:ty) => {
        impl Add<$x> for $y {
            type Output = Scalar<Building>;
            fn add(self, rhs: $x) -> Self::Output {
                sca_sca_core::add(self.as_scalar(), rhs.as_scalar()).clone()
            }
        }
        impl Sub<$x> for $y {
            type Output = Scalar<Building>;
            fn sub(self, rhs: $x) -> Self::Output {
                sca_sca_core::sub(self.as_scalar(), rhs.as_scalar()).clone()
            }
        }
        impl Mul<$x> for $y {
            type Output = Scalar<Building>;
            fn mul(self, rhs: $x) -> Self::Output {
                sca_sca_core::mul(self.as_scalar(), rhs.as_scalar()).clone()
            }
        }
        impl Div<$x> for $y {
            type Output = Scalar<Building>;
            fn div(self, rhs: $x) -> Self::Output {
                sca_sca_core::div(self.as_scalar(), rhs.as_scalar()).clone()
            }
        }
    };
}

impl_sca_sca_ops!(Scalar<Building>, Scalar<Building>);
impl_sca_sca_ops!(Scalar<Building>, Set<Building>);
impl_sca_sca_ops!(Scalar<Building>, Parameter<0, Building>);
impl_sca_sca_ops!(Set<Building>, Scalar<Building>);
impl_sca_sca_ops!(Set<Building>, Set<Building>);
impl_sca_sca_ops!(Set<Building>, Parameter<0, Building>);
impl_sca_sca_ops!(Parameter<0, Building>, Scalar<Building>);
impl_sca_sca_ops!(Parameter<0, Building>, Set<Building>);
impl_sca_sca_ops!(Parameter<0, Building>, Parameter<0, Building>);
