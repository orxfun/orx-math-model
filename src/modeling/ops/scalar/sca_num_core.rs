use super::core::new_scalar;
use crate::modeling::stages::Building;
use crate::{
    data_structs::current_set_indices::CurrentSetElements,
    modeling::{
        reference::HasRef,
        symbols::data::scalar_data::{ResolvedScalarVariant, ScalarVariant},
        symbols::scalar::Scalar,
    },
    numerics::{num::RealNum, ops::Ops},
    str_expr,
};
use std::{
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

macro_rules! ops {
    ($apply:expr, $ops:expr, $sca:expr, $real:expr, $as_sca_num:expr) => {{
        let variant = &$sca.data().variant;
        let core_ref = $sca.sym_ref().core_ref.clone();

        fn bin_num(num1: RealNum, num2: RealNum, correct_order: bool) -> RealNum {
            if correct_order {
                $apply(num1, num2)
            } else {
                $apply(num2, num1)
            }
        }
        fn bin_str(num1: &str, num2: &str, correct_order: bool) -> String {
            if correct_order {
                str_expr::binary_ops($ops, num1, num2)
            } else {
                str_expr::binary_ops($ops, num2, num1)
            }
        }

        let variant = match variant.as_resolved() {
            ResolvedScalarVariant::Real { str: _, num } => {
                ScalarVariant::new_from_num(bin_num(num, $real, $as_sca_num))
            }
            ResolvedScalarVariant::Func { str, fun } => {
                let fun = fun.clone();
                let str = bin_str(str, &$real.to_string(), $as_sca_num);
                let fun = Rc::new(move |indices: &CurrentSetElements| {
                    bin_num(fun(indices), $real, $as_sca_num)
                });
                ScalarVariant::Func { str, fun }
            }
        };

        new_scalar(core_ref, variant)
    }};
}

pub(crate) fn add(sca: Scalar<Building>, real: RealNum, as_sca_num: bool) -> Scalar<Building> {
    ops!(RealNum::add, Ops::Add, sca, real, as_sca_num)
}
pub(crate) fn sub(sca: Scalar<Building>, real: RealNum, as_sca_num: bool) -> Scalar<Building> {
    ops!(RealNum::sub, Ops::Sub, sca, real, as_sca_num)
}
pub(crate) fn mul(sca: Scalar<Building>, real: RealNum, as_sca_num: bool) -> Scalar<Building> {
    ops!(RealNum::mul, Ops::Mul, sca, real, as_sca_num)
}
pub(crate) fn div(sca: Scalar<Building>, real: RealNum, as_sca_num: bool) -> Scalar<Building> {
    ops!(RealNum::div, Ops::Div, sca, real, as_sca_num)
}
