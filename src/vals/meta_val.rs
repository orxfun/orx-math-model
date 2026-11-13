use crate::vals::{DVal, FVal, Int, Num, UFunVal};
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

pub trait Val: Clone + Copy + 'static {
    type Raw: Clone + Copy;

    type Sym<'m, 'd: 'm>: Clone
        + Copy
        + Deref<Target = Symbol<'m, 'd, DVal<'d>>>
        + From<Symbol<'m, 'd, DVal<'d>>>;

    fn typ() -> ValTyp;

    fn into_fun_union<'d>(fun: FVal<'d, Self>) -> UFunVal<'d>;

    fn fun_from_union<'a, 'd>(u: &'a UFunVal<'d>) -> &'a FVal<'d, Self>;

    fn new_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DVal<'d>>) -> Self::Sym<'m, 'd>;
}

// int

#[derive(Clone, Copy)]
pub struct ValInt;

impl Val for ValInt {
    type Raw = i64;

    type Sym<'m, 'd: 'm> = Int<'m, 'd>;

    fn typ() -> ValTyp {
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

    fn new_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DVal<'d>>) -> Self::Sym<'m, 'd> {
        Symbol::new(m, el).into()
    }
}

// num

#[derive(Clone, Copy)]
pub struct ValNum;

impl Val for ValNum {
    type Raw = f64;

    type Sym<'m, 'd: 'm> = Num<'m, 'd>;

    fn typ() -> ValTyp {
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

    fn new_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DVal<'d>>) -> Self::Sym<'m, 'd> {
        Symbol::new(m, el).into()
    }
}
