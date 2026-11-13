use crate::vals::{FVal, Int, Num, UFunVal};

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

    fn val_unpack<'a, 'd>(u: &'a UFunVal<'d>) -> &'a FVal<'d, Self>;
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

    fn val_unpack<'a, 'd>(u: &'a UFunVal<'d>) -> &'a FVal<'d, Self> {
        unsafe { &u.int }
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

    fn val_unpack<'a, 'd>(u: &'a UFunVal<'d>) -> &'a FVal<'d, Self> {
        unsafe { &u.num }
    }
}
