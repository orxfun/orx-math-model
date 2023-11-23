use super::sca_num_core;
use crate::{
    modeling::stages::Building,
    modeling::symbols::{
        conversions::scalar::AsScalar, parameter::Parameter, scalar::Scalar, set::Set,
    },
    numerics::num::Num,
};
use std::ops::{Add, Div, Mul, Sub};

macro_rules! impl_sca_num {
    ($sca:ty, $num:ty) => {
        impl Add<$num> for $sca {
            type Output = Scalar<Building>;
            fn add(self, rhs: $num) -> Self::Output {
                sca_num_core::add(self.as_scalar(), rhs.into_real_num(), true).clone()
            }
        }
        impl Sub<$num> for $sca {
            type Output = Scalar<Building>;
            fn sub(self, rhs: $num) -> Self::Output {
                sca_num_core::sub(self.as_scalar(), rhs.into_real_num(), true).clone()
            }
        }
        impl Mul<$num> for $sca {
            type Output = Scalar<Building>;
            fn mul(self, rhs: $num) -> Self::Output {
                sca_num_core::mul(self.as_scalar(), rhs.into_real_num(), true).clone()
            }
        }
        impl Div<$num> for $sca {
            type Output = Scalar<Building>;
            fn div(self, rhs: $num) -> Self::Output {
                sca_num_core::div(self.as_scalar(), rhs.into_real_num(), true).clone()
            }
        }
    };
}

macro_rules! impl_num {
    ($num:ty) => {
        impl_sca_num!(Scalar<Building>, $num);
        impl_sca_num!(Set<Building>, $num);
        impl_sca_num!(Parameter<0, Building>, $num);
    };
}

impl_num!(i64);
impl_num!(i32);
impl_num!(f64);
impl_num!(f32);
