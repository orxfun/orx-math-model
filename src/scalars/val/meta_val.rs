use crate::pars::{UFunP1, UFunP2, FP1, FP2};
use crate::scalars::val::{FVal, UFunVal};
use crate::scalars::{DSca, Sca};
use crate::{Model, Set, Symbol};
use core::mem::ManuallyDrop;
use orx_imp_vec::Elem;

// enums

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ValTyp {
    Int,
    Num,
}

impl ValTyp {
    fn to_int(x: f64) -> i64 {
        debug_assert!(x.is_finite());
        debug_assert!(x >= i64::MIN as f64);
        debug_assert!(x <= i64::MAX as f64);
        match x >= 0.0 {
            true => (x + 0.5) as i64,
            false => (x - 0.5) as i64,
        }
    }
}

// meta

pub trait Val: Sca {
    type Raw: Clone + Copy + Default;

    fn val_typ() -> ValTyp;

    fn minus_one() -> Self::Raw;

    fn into_fun_union<'d>(fun: FVal<'d, Self>) -> UFunVal<'d>;

    fn fun_from_union<'a, 'd>(u: &'a UFunVal<'d>) -> &'a FVal<'d, Self>;

    fn new_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DSca<'d>>) -> Self::Sym<'m, 'd>;

    // p1

    fn unpack_p1_fun<'a, 'd, I0: Set>(u: &'a UFunP1<'d>) -> &'a FP1<'d, Self, I0>;

    // p2

    fn unpack_p2_fun<'a, 'd, I0: Set, I1: Set>(u: &'a UFunP2<'d>) -> &'a FP2<'d, Self, I0, I1>;
}

// int

#[derive(Clone, Copy)]
pub struct ScaInt;

impl Val for ScaInt {
    type Raw = i64;

    fn minus_one() -> Self::Raw {
        -1
    }

    fn val_typ() -> ValTyp {
        ValTyp::Int
    }

    fn into_fun_union<'d>(fun: FVal<'d, Self>) -> UFunVal<'d> {
        UFunVal {
            int: ManuallyDrop::new(fun),
        }
    }

    #[inline(always)]
    fn fun_from_union<'a, 'd>(u: &'a UFunVal<'d>) -> &'a FVal<'d, Self> {
        unsafe { &u.int }
    }

    fn new_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DSca<'d>>) -> Self::Sym<'m, 'd> {
        Self::Sym::new(Symbol::new(m, el))
    }

    // p1

    #[inline(always)]
    fn unpack_p1_fun<'a, 'd, I0: Set>(u: &'a UFunP1<'d>) -> &'a FP1<'d, Self, I0> {
        I0::unpack_p1_fun_v(unsafe { &u.int })
    }

    // p2

    #[inline(always)]
    fn unpack_p2_fun<'a, 'd, I0: Set, I1: Set>(u: &'a UFunP2<'d>) -> &'a FP2<'d, Self, I0, I1> {
        I0::unpack_p2_fun_v::<Self, I1>(unsafe { &u.int })
    }
}

// num

#[derive(Clone, Copy)]
pub struct ScaNum;

impl Val for ScaNum {
    type Raw = f64;

    fn minus_one() -> Self::Raw {
        -1.0
    }

    fn val_typ() -> ValTyp {
        ValTyp::Num
    }

    fn into_fun_union<'d>(fun: FVal<'d, Self>) -> UFunVal<'d> {
        UFunVal {
            num: ManuallyDrop::new(fun),
        }
    }

    #[inline(always)]
    fn fun_from_union<'a, 'd>(u: &'a UFunVal<'d>) -> &'a FVal<'d, Self> {
        unsafe { &u.num }
    }

    fn new_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DSca<'d>>) -> Self::Sym<'m, 'd> {
        Self::Sym::new(Symbol::new(m, el))
    }

    // p1

    #[inline(always)]
    fn unpack_p1_fun<'a, 'd, I0: Set>(u: &'a UFunP1<'d>) -> &'a FP1<'d, Self, I0> {
        I0::unpack_p1_fun_v(unsafe { &u.num })
    }

    // p2

    #[inline(always)]
    fn unpack_p2_fun<'a, 'd, I0: Set, I1: Set>(u: &'a UFunP2<'d>) -> &'a FP2<'d, Self, I0, I1> {
        I0::unpack_p2_fun_v::<Self, I1>(unsafe { &u.num })
    }
}
