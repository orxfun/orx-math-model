use super::{super::super::ops::scalar::core::new_scalar, constraint_lhs::ConstraintLhs};
use crate::{
    modeling::stages::Building,
    modeling::{
        model_data_ref::ModelDataRef,
        symbols::{data::scalar_data::ScalarVariant, expression::Expression},
    },
    numerics::num::Num,
};

pub trait ConstraintRhs {
    fn to_rhs_expression(&self, core_ref: ModelDataRef<Building>) -> Expression<Building>;
}

impl<T: ConstraintLhs> ConstraintRhs for T {
    fn to_rhs_expression(&self, _: ModelDataRef<Building>) -> Expression<Building> {
        self.to_lhs_expression()
    }
}

macro_rules! impl_num {
    ($num:ty) => {
        impl ConstraintRhs for $num {
            fn to_rhs_expression(&self, core_ref: ModelDataRef<Building>) -> Expression<Building> {
                let variant = ScalarVariant::new_from_num(self.into_real_num());
                let scalar = new_scalar(core_ref, variant);
                scalar.to_lhs_expression()
            }
        }
    };
}
impl_num!(i64);
impl_num!(i32);
impl_num!(f64);
impl_num!(f32);
