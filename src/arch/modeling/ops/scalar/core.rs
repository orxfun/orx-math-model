use crate::modeling::model_data_ref::ModelDataRef;
use crate::modeling::symbol_collections::scalars::Scalars;
use crate::numerics::num::Num;
use crate::{
    modeling::stages::Building,
    modeling::symbols::{data::scalar_data::ScalarVariant, scalar::Scalar},
};

pub(crate) fn new_scalar(
    core_ref: ModelDataRef<Building>,
    variant: ScalarVariant,
) -> Scalar<Building> {
    use ScalarVariant::*;
    match variant {
        One => core_ref.core().scalars.one(),
        MinusOne => core_ref.core().scalars.minus_one(),
        Zero => core_ref.core().scalars.zero(),
        Real { str: _, num } => {
            if num.is_one() {
                core_ref.core().scalars.one()
            } else if num.is_minus_one() {
                core_ref.core().scalars.minus_one()
            } else if num.is_zero() {
                core_ref.core().scalars.zero()
            } else {
                Scalars::push_new_scalar(core_ref, variant)
            }
        }
        _ => Scalars::push_new_scalar(core_ref, variant),
    }
}
