use super::core::new_scalar;
use crate::modeling::stages::Building;
use crate::{
    data_structs::current_set_indices::CurrentSetElements,
    modeling::{
        reference::HasRef,
        symbols::data::scalar_data::{ResolvedScalarVariant, ScalarVariant},
        symbols::{conversions::scalar::AsScalar, scalar::Scalar, set::Set},
    },
    str_expr,
};
use std::{ops::Neg, rc::Rc};

macro_rules! impl_neg {
    ($sca:ty) => {
        impl Neg for $sca {
            type Output = Scalar<Building>;
            fn neg(self) -> Self::Output {
                let scalar = self.as_scalar();
                let variant = &scalar.data().variant;

                let variant = match variant.as_resolved() {
                    ResolvedScalarVariant::Real { str: _, num } => ScalarVariant::new_from_num(num),
                    ResolvedScalarVariant::Func { str, fun } => {
                        let fun = fun.clone();
                        let negstr = str_expr::neg(str);
                        let negfun = Rc::new(move |indices: &CurrentSetElements| -fun(indices));
                        ScalarVariant::Func {
                            str: negstr,
                            fun: negfun,
                        }
                    }
                };

                new_scalar(scalar.sym_ref().core_ref.clone(), variant)
            }
        }
    };
}

impl_neg!(Scalar<Building>);
impl_neg!(Set<Building>);
