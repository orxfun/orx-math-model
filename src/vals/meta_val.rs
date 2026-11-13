use crate::vals::{DVal, FVal, Int, Num, UFunVal};
use crate::{Model, Symbol};
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

    type Sym<'m, 'd: 'm>: Clone + Copy;

    fn typ() -> ValTyp;

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

    fn fun_from_union<'a, 'd>(u: &'a UFunVal<'d>) -> &'a FVal<'d, Self> {
        unsafe { &u.num }
    }

    fn new_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, DVal<'d>>) -> Self::Sym<'m, 'd> {
        Symbol::new(m, el).into()
    }
}
