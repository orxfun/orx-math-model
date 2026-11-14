use crate::pars::p1::{ParD1, UFunP1, DP1, FP1};
use crate::{Model, Set, SetCatInt, SetCatStr, SetOrd, Symbol, Val, ValInt, ValNum};
use core::mem::ManuallyDrop;
use orx_imp_vec::Elem;

pub trait D1ValIdxPair {
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

impl D1ValIdxPair for (ValInt, SetCatInt) {
    type V = ValInt;

    type I0 = SetCatInt;

    fn into_fun_union<'d>(fun: FP1<'d, Self::V, Self::I0>) -> UFunP1<'d> {
        UFunP1 {
            int_cint: ManuallyDrop::new(fun),
        }
    }

    fn fun_from_union<'a, 'd>(data: &'a UFunP1<'d>) -> &'a FP1<'d, Self::V, Self::I0> {
        unsafe { &data.int_cint }
    }
}

impl D1ValIdxPair for (ValInt, SetCatStr) {
    type V = ValInt;

    type I0 = SetCatStr;

    fn into_fun_union<'d>(fun: FP1<'d, Self::V, Self::I0>) -> UFunP1<'d> {
        UFunP1 {
            int_cstr: ManuallyDrop::new(fun),
        }
    }

    fn fun_from_union<'a, 'd>(data: &'a UFunP1<'d>) -> &'a FP1<'d, Self::V, Self::I0> {
        unsafe { &data.int_cstr }
    }
}

impl D1ValIdxPair for (ValInt, SetOrd) {
    type V = ValInt;

    type I0 = SetOrd;

    fn into_fun_union<'d>(fun: FP1<'d, Self::V, Self::I0>) -> UFunP1<'d> {
        UFunP1 {
            int_ord: ManuallyDrop::new(fun),
        }
    }

    fn fun_from_union<'a, 'd>(data: &'a UFunP1<'d>) -> &'a FP1<'d, Self::V, Self::I0> {
        unsafe { &data.int_ord }
    }
}

// num

impl D1ValIdxPair for (ValNum, SetCatInt) {
    type V = ValNum;

    type I0 = SetCatInt;

    fn into_fun_union<'d>(fun: FP1<'d, Self::V, Self::I0>) -> UFunP1<'d> {
        UFunP1 {
            num_cint: ManuallyDrop::new(fun),
        }
    }

    fn fun_from_union<'a, 'd>(data: &'a UFunP1<'d>) -> &'a FP1<'d, Self::V, Self::I0> {
        unsafe { &data.num_cint }
    }
}

impl D1ValIdxPair for (ValNum, SetCatStr) {
    type V = ValNum;

    type I0 = SetCatStr;

    fn into_fun_union<'d>(fun: FP1<'d, Self::V, Self::I0>) -> UFunP1<'d> {
        UFunP1 {
            num_cstr: ManuallyDrop::new(fun),
        }
    }

    fn fun_from_union<'a, 'd>(data: &'a UFunP1<'d>) -> &'a FP1<'d, Self::V, Self::I0> {
        unsafe { &data.num_cstr }
    }
}

impl D1ValIdxPair for (ValNum, SetOrd) {
    type V = ValNum;

    type I0 = SetOrd;

    fn into_fun_union<'d>(fun: FP1<'d, Self::V, Self::I0>) -> UFunP1<'d> {
        UFunP1 {
            num_ord: ManuallyDrop::new(fun),
        }
    }

    fn fun_from_union<'a, 'd>(data: &'a UFunP1<'d>) -> &'a FP1<'d, Self::V, Self::I0> {
        unsafe { &data.num_ord }
    }
}
