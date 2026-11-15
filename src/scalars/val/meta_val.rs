use crate::scalars::val::{FVal, Int, Num, UFunVal};
use crate::scalars::{DSca, Sca};
use crate::{Model, Symbol};
use core::mem::{transmute, ManuallyDrop};
use core::ops::Deref;
use orx_imp_vec::Elem;

// enums

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ValTyp {
    Int,
    Num,
}

// meta

pub trait Val: Sca {
    type Raw: Clone + Copy + Default;

    fn val_typ() -> ValTyp;

    fn into_fun_union<'d>(fun: FVal<'d, Self>) -> UFunVal<'d>;

    fn fun_from_union<'a, 'd>(u: &'a UFunVal<'d>) -> &'a FVal<'d, Self>;

    fn new_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DSca<'d>>) -> Self::Sym<'m, 'd>;
}

// int

#[derive(Clone, Copy)]
pub struct ScaInt;

impl Val for ScaInt {
    type Raw = i64;

    fn val_typ() -> ValTyp {
        ValTyp::Int
    }

    fn into_fun_union<'d>(fun: FVal<'d, Self>) -> UFunVal<'d> {
        UFunVal {
            int: ManuallyDrop::new(fun),
        }
    }

    fn fun_from_union<'a, 'd>(u: &'a UFunVal<'d>) -> &'a FVal<'d, Self> {
        unsafe { &u.int }
    }

    fn new_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DSca<'d>>) -> Self::Sym<'m, 'd> {
        Symbol::new(m, el).into()
    }
}

// num

#[derive(Clone, Copy)]
pub struct ScaNum;

impl Val for ScaNum {
    type Raw = f64;

    fn val_typ() -> ValTyp {
        ValTyp::Num
    }

    fn into_fun_union<'d>(fun: FVal<'d, Self>) -> UFunVal<'d> {
        UFunVal {
            num: ManuallyDrop::new(fun),
        }
    }

    fn fun_from_union<'a, 'd>(u: &'a UFunVal<'d>) -> &'a FVal<'d, Self> {
        unsafe { &u.num }
    }

    fn new_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DSca<'d>>) -> Self::Sym<'m, 'd> {
        Symbol::new(m, el).into()
    }
}
