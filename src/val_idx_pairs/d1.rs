use crate::par::p1::{UFunP1, FP1};
use crate::{Set, SetCatInt, SetCatStr, SetOrd, Val, ValInt, ValNum};

pub trait ValIdxD1 {
    type V: Val;

    type I0: Set;

    fn p1_fun_unpack<'a, 'd>(data: &'a UFunP1<'d>) -> &'a FP1<'d, Self::V, Self::I0>;
}

// int

impl ValIdxD1 for (ValInt, SetCatInt) {
    type V = ValInt;

    type I0 = SetCatInt;

    fn p1_fun_unpack<'a, 'd>(data: &'a UFunP1<'d>) -> &'a FP1<'d, Self::V, Self::I0> {
        unsafe { &data.cint_int }
    }
}

impl ValIdxD1 for (ValInt, SetCatStr) {
    type V = ValInt;

    type I0 = SetCatStr;

    fn p1_fun_unpack<'a, 'd>(data: &'a UFunP1<'d>) -> &'a FP1<'d, Self::V, Self::I0> {
        unsafe { &data.cstr_int }
    }
}

impl ValIdxD1 for (ValInt, SetOrd) {
    type V = ValInt;

    type I0 = SetOrd;

    fn p1_fun_unpack<'a, 'd>(data: &'a UFunP1<'d>) -> &'a FP1<'d, Self::V, Self::I0> {
        unsafe { &data.ord_int }
    }
}

// num

impl ValIdxD1 for (ValNum, SetCatInt) {
    type V = ValNum;

    type I0 = SetCatInt;

    fn p1_fun_unpack<'a, 'd>(data: &'a UFunP1<'d>) -> &'a FP1<'d, Self::V, Self::I0> {
        unsafe { &data.cint_num }
    }
}

impl ValIdxD1 for (ValNum, SetCatStr) {
    type V = ValNum;

    type I0 = SetCatStr;

    fn p1_fun_unpack<'a, 'd>(data: &'a UFunP1<'d>) -> &'a FP1<'d, Self::V, Self::I0> {
        unsafe { &data.cstr_num }
    }
}

impl ValIdxD1 for (ValNum, SetOrd) {
    type V = ValNum;

    type I0 = SetOrd;

    fn p1_fun_unpack<'a, 'd>(data: &'a UFunP1<'d>) -> &'a FP1<'d, Self::V, Self::I0> {
        unsafe { &data.ord_num }
    }
}
