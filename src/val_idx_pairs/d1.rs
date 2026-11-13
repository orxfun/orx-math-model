use crate::pars::p1::{ParD1, UFunP1, DP1, FP1};
use crate::{Model, Set, SetCatInt, SetCatStr, SetOrd, Symbol, Val, ValInt, ValNum};
use core::mem::ManuallyDrop;
use orx_imp_vec::Elem;

pub trait ValIdxPairD1 {
    type V: Val;

    type I0: Set;

    fn into_fun_union<'d>(fun: FP1<'d, Self::V, Self::I0>) -> UFunP1<'d>;

    fn fun_from_union<'a, 'd>(data: &'a UFunP1<'d>) -> &'a FP1<'d, Self::V, Self::I0>;

    fn new_p1_sym<'m, 'd>(
        m: &'m Model<'d>,
        el: Elem<'m, DP1<'d>>,
    ) -> ParD1<'m, 'd, Self::V, Self::I0>
    where
        ParD1<'m, 'd, Self::V, Self::I0>: From<Symbol<'m, 'd, DP1<'d>>>,
    {
        Symbol::new(m, el).into()
    }
}

// int

impl ValIdxPairD1 for (ValInt, SetCatInt) {
    type V = ValInt;

    type I0 = SetCatInt;

    fn into_fun_union<'d>(fun: FP1<'d, Self::V, Self::I0>) -> UFunP1<'d> {
        UFunP1 {
            cint_int: ManuallyDrop::new(fun),
        }
    }

    fn fun_from_union<'a, 'd>(data: &'a UFunP1<'d>) -> &'a FP1<'d, Self::V, Self::I0> {
        unsafe { &data.cint_int }
    }
}

impl ValIdxPairD1 for (ValInt, SetCatStr) {
    type V = ValInt;

    type I0 = SetCatStr;

    fn into_fun_union<'d>(fun: FP1<'d, Self::V, Self::I0>) -> UFunP1<'d> {
        UFunP1 {
            cstr_int: ManuallyDrop::new(fun),
        }
    }

    fn fun_from_union<'a, 'd>(data: &'a UFunP1<'d>) -> &'a FP1<'d, Self::V, Self::I0> {
        unsafe { &data.cstr_int }
    }
}

impl ValIdxPairD1 for (ValInt, SetOrd) {
    type V = ValInt;

    type I0 = SetOrd;

    fn into_fun_union<'d>(fun: FP1<'d, Self::V, Self::I0>) -> UFunP1<'d> {
        UFunP1 {
            ord_int: ManuallyDrop::new(fun),
        }
    }

    fn fun_from_union<'a, 'd>(data: &'a UFunP1<'d>) -> &'a FP1<'d, Self::V, Self::I0> {
        unsafe { &data.ord_int }
    }
}

// num

impl ValIdxPairD1 for (ValNum, SetCatInt) {
    type V = ValNum;

    type I0 = SetCatInt;

    fn into_fun_union<'d>(fun: FP1<'d, Self::V, Self::I0>) -> UFunP1<'d> {
        UFunP1 {
            cint_num: ManuallyDrop::new(fun),
        }
    }

    fn fun_from_union<'a, 'd>(data: &'a UFunP1<'d>) -> &'a FP1<'d, Self::V, Self::I0> {
        unsafe { &data.cint_num }
    }
}

impl ValIdxPairD1 for (ValNum, SetCatStr) {
    type V = ValNum;

    type I0 = SetCatStr;

    fn into_fun_union<'d>(fun: FP1<'d, Self::V, Self::I0>) -> UFunP1<'d> {
        UFunP1 {
            cstr_num: ManuallyDrop::new(fun),
        }
    }

    fn fun_from_union<'a, 'd>(data: &'a UFunP1<'d>) -> &'a FP1<'d, Self::V, Self::I0> {
        unsafe { &data.cstr_num }
    }
}

impl ValIdxPairD1 for (ValNum, SetOrd) {
    type V = ValNum;

    type I0 = SetOrd;

    fn into_fun_union<'d>(fun: FP1<'d, Self::V, Self::I0>) -> UFunP1<'d> {
        UFunP1 {
            ord_num: ManuallyDrop::new(fun),
        }
    }

    fn fun_from_union<'a, 'd>(data: &'a UFunP1<'d>) -> &'a FP1<'d, Self::V, Self::I0> {
        unsafe { &data.ord_num }
    }
}
