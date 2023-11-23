use super::core::new_scalar;
use crate::modeling::stages::Building;
use crate::{
    data_structs::current_set_indices::CurrentSetElements,
    modeling::{
        reference::HasRef,
        symbols::{
            data::scalar_data::{ResolvedScalarVariant, ScalarVariant},
            scalar::Scalar,
        },
    },
    numerics::{
        num::{Num, RealNum},
        ops::Ops,
    },
    str_expr,
};
use std::ops::{Add, Div, Mul, Sub};
use std::rc::Rc;
use ResolvedScalarVariant::*;

macro_rules! ops {
    ($apply:expr, $ops:expr, $lhs:expr, $rhs:expr) => {{
        let left = $lhs.data().variant.as_resolved();
        let right = $rhs.data().variant.as_resolved();

        let variant = match left {
            Real {
                str: str1,
                num: num1,
            } => match right {
                Real { str: _, num: num2 } => {
                    let num = $apply(num1, num2);
                    if num.is_zero() {
                        ScalarVariant::Zero
                    } else if num.is_one() {
                        ScalarVariant::One
                    } else if num.is_minus_one() {
                        ScalarVariant::MinusOne
                    } else {
                        ScalarVariant::Real {
                            str: num.to_string(),
                            num,
                        }
                    }
                }
                Func {
                    str: str2,
                    fun: fun2,
                } => {
                    let str = str_expr::binary_ops($ops, str1, str2);
                    let fun =
                        Rc::new(move |indices: &CurrentSetElements| $apply(num1, fun2(indices)));
                    ScalarVariant::Func { str, fun }
                }
            },
            Func {
                str: str1,
                fun: fun1,
            } => match right {
                Real {
                    str: str2,
                    num: num2,
                } => {
                    let str = str_expr::binary_ops($ops, str1, str2);
                    let fun =
                        Rc::new(move |indices: &CurrentSetElements| $apply(fun1(indices), num2));
                    ScalarVariant::Func { str, fun }
                }
                Func {
                    str: str2,
                    fun: fun2,
                } => {
                    let str = str_expr::binary_ops($ops, str1, str2);
                    let fun = Rc::new(move |indices: &CurrentSetElements| {
                        $apply(fun1(indices), fun2(indices))
                    });
                    ScalarVariant::Func { str, fun }
                }
            },
        };

        new_scalar($lhs.sym_ref().core_ref.clone(), variant)
    }};
}

pub(crate) fn add(lhs: Scalar<Building>, rhs: Scalar<Building>) -> Scalar<Building> {
    ops!(RealNum::add, Ops::Add, lhs, rhs)
}
pub(crate) fn sub(lhs: Scalar<Building>, rhs: Scalar<Building>) -> Scalar<Building> {
    ops!(RealNum::sub, Ops::Sub, lhs, rhs)
}
pub(crate) fn mul(lhs: Scalar<Building>, rhs: Scalar<Building>) -> Scalar<Building> {
    ops!(RealNum::mul, Ops::Mul, lhs, rhs)
}
pub(crate) fn div(lhs: Scalar<Building>, rhs: Scalar<Building>) -> Scalar<Building> {
    ops!(RealNum::div, Ops::Div, lhs, rhs)
}
