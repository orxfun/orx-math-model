use super::sca_num_core;
use crate::{
    modeling::stages::Building,
    modeling::symbols::{
        conversions::scalar::AsScalar, parameter::Parameter, scalar::Scalar, set::Set,
    },
    numerics::num::Num,
};
use std::ops::{Add, Div, Mul, Sub};

macro_rules! impl_num_sca {
    ($num:ty, $sca:ty) => {
        impl Add<$sca> for $num {
            type Output = Scalar<Building>;
            fn add(self, rhs: $sca) -> Self::Output {
                sca_num_core::add(rhs.as_scalar(), self.into_real_num(), false).clone()
            }
        }
        impl Sub<$sca> for $num {
            type Output = Scalar<Building>;
            fn sub(self, rhs: $sca) -> Self::Output {
                sca_num_core::sub(rhs.as_scalar(), self.into_real_num(), false).clone()
            }
        }
        impl Mul<$sca> for $num {
            type Output = Scalar<Building>;
            fn mul(self, rhs: $sca) -> Self::Output {
                sca_num_core::mul(rhs.as_scalar(), self.into_real_num(), false).clone()
            }
        }
        impl Div<$sca> for $num {
            type Output = Scalar<Building>;
            fn div(self, rhs: $sca) -> Self::Output {
                sca_num_core::div(rhs.as_scalar(), self.into_real_num(), false).clone()
            }
        }
    };
}

macro_rules! impl_num {
    ($num:ty) => {
        impl_num_sca!($num, Scalar<Building>);
        impl_num_sca!($num, Set<Building>);
        impl_num_sca!($num, Parameter<0, Building>);
    };
}

impl_num!(i64);
impl_num!(i32);
impl_num!(f64);
impl_num!(f32);
