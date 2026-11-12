use crate::vals::{Int, Num, ValFun};
use crate::{Model, Symbol};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// enums

pub enum ValTyp {
    Int,
    Num,
}

#[derive(Debug, PartialEq)]
pub enum ValVal {
    Int(i64),
    Num(f64),
}

pub enum ValSymSum<'m, 'd> {
    Int(Int<'m, 'd>),
    Num(Num<'m, 'd>),
}

// trait

pub trait MetaVal: Clone + Copy + 'static {
    type Val: Default + Clone + Copy;

    type Sym<'m, 'd>: Val<'m, 'd>
    where
        'd: 'm;

    fn val_typ() -> ValTyp;

    fn val_as_enum(idx: Self::Val) -> ValVal;

    fn create_val_fun<'d>(fun: impl Fn() -> Self::Val + 'd) -> ValFun<'d>;

    fn create_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, ValFun<'d>>) -> Self::Sym<'m, 'd>;
}

// int

#[derive(Clone, Copy)]
pub struct MetaInt;

impl MetaVal for MetaInt {
    type Val = i64;

    type Sym<'m, 'd>
        = Int<'m, 'd>
    where
        'd: 'm;

    fn val_typ() -> ValTyp {
        ValTyp::Int
    }

    fn val_as_enum(idx: Self::Val) -> ValVal {
        ValVal::Int(idx)
    }

    fn create_val_fun<'d>(fun: impl Fn() -> Self::Val + 'd) -> ValFun<'d> {
        ValFun::Int(Box::new(fun))
    }

    fn create_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, ValFun<'d>>) -> Self::Sym<'m, 'd> {
        Symbol::new(m, el).into()
    }
}

// num

#[derive(Clone, Copy)]
pub struct MetaNum;

impl MetaVal for MetaNum {
    type Val = f64;

    type Sym<'m, 'd>
        = Num<'m, 'd>
    where
        'd: 'm;

    fn val_typ() -> ValTyp {
        ValTyp::Num
    }

    fn val_as_enum(idx: Self::Val) -> ValVal {
        ValVal::Num(idx)
    }

    fn create_val_fun<'d>(fun: impl Fn() -> Self::Val + 'd) -> ValFun<'d> {
        ValFun::Num(Box::new(fun))
    }

    fn create_sym<'m, 'd>(m: &'m Model<'d>, el: Elem<'m, ValFun<'d>>) -> Self::Sym<'m, 'd> {
        Symbol::new(m, el).into()
    }
}

// symbols

pub trait Val<'m, 'd> {
    fn into_sym_sum(self) -> ValSymSum<'m, 'd>;

    fn from_sum_to_self(sum: &'m ValSymSum<'m, 'd>) -> &'m Self;
}

impl<'m, 'd> Val<'m, 'd> for Int<'m, 'd> {
    fn into_sym_sum(self) -> ValSymSum<'m, 'd> {
        ValSymSum::Int(self)
    }

    fn from_sum_to_self(sum: &'m ValSymSum<'m, 'd>) -> &'m Self {
        match sum {
            ValSymSum::Int(x) => x,
            _ => unreachable!(),
        }
    }
}

impl<'m, 'd> Val<'m, 'd> for Num<'m, 'd> {
    fn into_sym_sum(self) -> ValSymSum<'m, 'd> {
        ValSymSum::Num(self)
    }

    fn from_sum_to_self(sum: &'m ValSymSum<'m, 'd>) -> &'m Self {
        match sum {
            ValSymSum::Num(x) => x,
            _ => unreachable!(),
        }
    }
}
